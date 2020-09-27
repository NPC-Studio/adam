use crate::gm_artifacts;

use super::run::RunCommand;
use heck::TitleCase;
use indicatif::ProgressBar;
use std::{io::BufRead, io::BufReader, path::Path, process::Child};

pub struct CompilerHandler(CompilerState, bool);

enum CompilerState {
    Initialize,
    Compile(Vec<String>),
    ChunkBuilder,
    PreRunToMainLoop(Vec<String>),
}

impl CompilerHandler {
    pub fn new_run() -> Self {
        Self(CompilerState::Initialize, false)
    }

    pub fn new_build() -> Self {
        Self(CompilerState::Initialize, true)
    }

    pub fn new_rerun() -> Self {
        Self(CompilerState::PreRunToMainLoop(vec![]), false)
    }

    pub fn compile(
        mut self,
        child: &mut Child,
        project_name: &str,
        project_path: &Path,
        run_command: RunCommand,
    ) -> CompilerOutput {
        let progress_bar = ProgressBar::new(1000);
        progress_bar.set_draw_target(indicatif::ProgressDrawTarget::stdout());
        progress_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")
                .progress_chars("#> "),
        );
        progress_bar.enable_steady_tick(100);
        progress_bar.println(format!(
            "{} {} ({})",
            console::style("Compiling").green().bright(),
            project_name.to_title_case(),
            project_path.display()
        ));

        let start_time = std::time::Instant::now();
        let lines = BufReader::new(child.stdout.as_mut().unwrap()).lines();

        for line in lines.filter_map(|v| v.ok()) {
            let max_size = line.len().min(30);

            match &mut self.0 {
                CompilerState::Initialize => {
                    progress_bar.set_message(&line[..max_size]);

                    if line.contains("[Compile]") {
                        self.0 = CompilerState::Compile(vec![]);
                        progress_bar.set_position(progress_bar.position().max(250));
                    } else {
                        progress_bar.inc(20);
                    }
                }
                CompilerState::Compile(e_msgs) => {
                    if line.contains("Error") {
                        e_msgs.push(line);
                        progress_bar.set_message("Collecting errors...");
                    } else if line.contains("Final Compile...finished") {
                        progress_bar.set_position(progress_bar.position().max(500));
                        if e_msgs.is_empty() {
                            self.0 = CompilerState::ChunkBuilder;
                        } else {
                            return CompilerOutput::Errors(e_msgs.clone());
                        }
                    } else if e_msgs.is_empty() {
                        progress_bar.set_message(&line[..max_size]);
                    } else {
                        progress_bar.inc(20);
                    }
                }
                CompilerState::ChunkBuilder => {
                    #[cfg(target_os = "windows")]
                    const CHUNK_ENDER: &str = "Igor complete";

                    #[cfg(not(target_os = "windows"))]
                    const CHUNK_ENDER: &str = "Finished PrepareGame()";

                    // we're in the final stage...
                    if line.contains(CHUNK_ENDER) {
                        progress_bar.set_message("adam compile complete");
                        if self.1 {
                            progress_bar.finish_and_clear();
                            if let Err(e) = child.kill() {
                                println!(
                                    "{}: could not kill the compiler process, {}",
                                    console::style("error").red().bright(),
                                    e
                                );
                            }
                            progress_bar.finish_and_clear();
                            println!(
                                "{} {} {}:{} in {}",
                                console::style("Completed").green().bright(),
                                gm_artifacts::PLATFORM.to_string(),
                                run_command,
                                console::style(&run_command.1.config).yellow().bright(),
                                indicatif::HumanDuration(std::time::Instant::now() - start_time)
                            );

                            return CompilerOutput::SuccessAndBuild;
                        } else {
                            progress_bar.set_position(progress_bar.position().max(750));
                            self.0 = CompilerState::PreRunToMainLoop(vec![]);
                        }
                    } else {
                        progress_bar.set_message(&line[..max_size]);
                        progress_bar.inc(10);
                    }
                }
                CompilerState::PreRunToMainLoop(startup_msgs) => {
                    const FINAL_EMITS: [&str; 10] = [
                        "Run_Start",
                        "[Run]",
                        "MainOptions.json",
                        "gamepadcount",
                        "hardware device",
                        "Collision Event time",
                        "Entering main loop.",
                        "Total memory used",
                        "Texture #",
                        "********",
                    ];

                    if line == "Entering main loop." {
                        progress_bar.finish_and_clear();
                        println!(
                            "{} {} {}:{} in {}",
                            console::style("Completed").green().bright(),
                            gm_artifacts::PLATFORM.to_string(),
                            run_command,
                            console::style(&run_command.1.config).yellow().bright(),
                            indicatif::HumanDuration(std::time::Instant::now() - start_time)
                        );

                        return CompilerOutput::SuccessAndRun(startup_msgs.clone());
                    } else {
                        // we're in the final stage...
                        if FINAL_EMITS.iter().any(|&v| line.contains(v)) == false {
                            startup_msgs.push(line);
                        } else {
                            progress_bar.set_message(&line);
                            progress_bar.inc(25);
                        }
                    }
                }
            }
        }

        CompilerOutput::Errors(vec![
            "adam error: unexpected end of compiler messages. Are you on an unsupported platform?"
                .to_string(),
        ])
    }
}

pub enum CompilerOutput {
    Errors(Vec<String>),
    SuccessAndBuild,
    SuccessAndRun(Vec<String>),
}

/*
Looking for built-in particle images in /Users/Shared/GameMakerStudio2/Cache/runtimes/runtime-2.3.0.401/bin/ParticleImages
Saving IFF file... /Users/jjspira/Documents/GMS2/SwordAndField/target/vm/FieldsOfMistria.zip
Writing Chunk... GEN8
option_game_speed=60
Writing Chunk... OPTN
Writing Chunk... LANG
Writing Chunk... EXTN
Writing Chunk... SOND
Writing Chunk... AGRP
Writing Chunk... SPRT
Writing Chunk... BGND
Writing Chunk... PATH
Writing Chunk... SCPT
Writing Chunk... GLOB
Writing Chunk... SHDR
Writing Chunk... FONT
Writing Chunk... TMLN
Writing Chunk... OBJT
Writing Chunk... ACRV
Writing Chunk... SEQN
Writing Chunk... TAGS
Writing Chunk... ROOM
Writing Chunk... DAFL
Writing Chunk... EMBI
Writing Chunk... TPAGE
Texture Group - __YY__0map_farm_YYG_AUTO_GEN_TEX_GROUP_NAME_0
Texture Group - Default
Writing Chunk... TGIN
Writing Chunk... CODE
Writing Chunk... VARI
Writing Chunk... FUNC
Writing Chunk... STRG
Writing Chunk... TXTR
0 Compressing texture... writing texture texture_0.png...
1 Compressing texture... writing texture texture_1.png...
2 Compressing texture... writing texture texture_2.png...
3 Compressing texture... writing texture texture_3.png...
4 Compressing texture... writing texture texture_4.png...
Writing Chunk... AUDO
Writing Chunk... SCPT
Writing Chunk... DBGI
Writing Chunk... INST
Writing Chunk... LOCL
Writing Chunk... DFNC
Writing Chunk... STRG
Writing Chunk... SCPT
Writing Chunk... DBGI
Writing Chunk... INST
Writing Chunk... LOCL
Writing Chunk... DFNC
Writing Chunk... STRG
Stats : GMA : Elapsed=14539.081
Stats : GMA : sp=821,au=33,bk=40,pt=0,sc=1624,sh=11,fo=3,tl=0,ob=104,ro=36,da=13,ex=0,ma=360,fm=0xD000B39D7FFE22B4
nname: /Users/jjspira/Documents/GMS2/SwordAndField/target/vm/FieldsOfMistria.zip
destname: /Users/jjspira/Documents/GMS2/SwordAndField/target/vm/GameAssetsMac.zip
IsMacConnected
InstallRunnerOnMac
/bin/bash -c "chmod u+x /Users/jjspira/Documents/GMS2/SwordAndField/target/vm/install_dmg.sh"


/bin/bash DONE (0)
Successfully extracted game.yydebug
RunOnMac
/bin/bash -c "cd ~ && mkdir -p \"/Users/jjspira/GameMakerStudio2/Mac/GMS2MAC/FieldsOfMistria\""


/bin/bash DONE (0)
/bin/bash -c "cd ~ && echo Starting... >/Users/jjspira/Documents/GMS2/SwordAndField/target/vm/debug.log"


/bin/bash DONE (0)
-n -a "/Users/Shared/GameMakerStudio2/Cache/runtimes/runtime-2.3.0.401/mac/YoYo Runner.app" --args -game "/Users/jjspira/Documents/GMS2/SwordAndField/target/vm/GameAssetsMac.zip" -debugoutput "/Users/jjspira/Documents/GMS2/SwordAndField/target/vm/debug.log" -output "/Users/jjspira/Documents/GMS2/SwordAndField/target/vm/debug.log" -runTest
Igor complete.
Starting...

***************************************
*     YoYo Games Mac Runner V0.1      *
***************************************
CommandLine: -game /Users/jjspira/Documents/GMS2/SwordAndField/target/vm/GameAssetsMac.zip -debugoutput /Users/jjspira/Documents/GMS2/SwordAndField/target/vm/debug.log -output /Users/jjspira/Documents/GMS2/SwordAndField/target/vm/debug.log -runTest -game "/Users/jjspira/Library/Application Support/com.yoyogames.macyoyorunner/game/assets/game.ios" -debugoutput "/Users/jjspira/Documents/GMS2/SwordAndField/target/vm/debug.log" -output "/Users/jjspira/Documents/GMS2/SwordAndField/target/vm/debug.log"
MemoryManager allocated: 422517
Processing command line -game /Users/jjspira/Documents/GMS2/SwordAndField/target/vm/GameAssetsMac.zip -debugoutput /Users/jjspira/Documents/GMS2/SwordAndField/target/vm/debug.log -output /Users/jjspira/Documents/GMS2/SwordAndField/target/vm/debug.log -runTest -game "/Users/jjspira/Library/Application Support/com.yoyogames.macyoyorunner/game/assets/game.ios" -debugoutput "/Users/jjspira/Documents/GMS2/SwordAndField/target/vm/debug.log" -output "/Users/jjspira/Documents/GMS2/SwordAndField/target/vm/debug.log"
Create Error Form

***************************************
*     YoYo Games Runner v1.0(401)[r32908]    *
***************************************
RunnerLoadGame: /Users/jjspira/Library/Application Support/com.yoyogames.macyoyorunner/game/assets/game.ios
#########################################################################
####!!!!$$$$$$ pwd - /Users/jjspira/Library/Application Support/com.yoyogames.macyoyorunner/game/assets/
#########################################################################
RunnerLoadGame() - /Users/jjspira/Library/Application Support/com.yoyogames.macyoyorunner/game/assets/game.ios
YYG Game launching. Game file: /Users/jjspira/Library/Application Support/com.yoyogames.macyoyorunner/game/assets/game.ios
Checking if INIFile exists at /Users/jjspira/Library/Application Support/com.yoyogames.macyoyorunner/game/assets/options.ini
INIFile /Users/jjspira/Library/Application Support/com.yoyogames.macyoyorunner/game/assets/options.ini Exists, loading....
Process Chunk: SCPT   16256
Process Chunk: DBGI   396296
Process Chunk: INST   328
Process Chunk: LOCL   21272
Process Chunk: DFNC   48296
Process Chunk: STRG   874826
Reading File /Users/jjspira/Library/Application Support/com.yoyogames.macyoyorunner/game/assets/game.ios
Loaded File /Users/jjspira/Library/Application Support/com.yoyogames.macyoyorunner/game/assets/game.ios(86847020)
IFF wad found
Get Resolution
Get Header Information
InitGMLFunctions
HighScore..filename is /Users/jjspira/Library/Application Support/com.yoyogames.macyoyorunner/hiscore.dat
Game..Math..Graphic..Action..File..Resource..Interaction..3D..Particle..Misc..DS..Sound..Physics..Gamepad..Attempting to set gamepadcount to 4
libYoYoGamepad.dylib found at path /Users/Shared/GameMakerStudio2/Cache/runtimes/runtime-2.3.0.401/mac/YoYo Runner.app/Contents/MacOS/../Frameworks/libYoYoGamepad.dylib
Buffers..Networking..Shaders..InitPushFunctions...YoYo..filename is /Users/jjspira/Library/Application Support/com.yoyogames.macyoyorunner/playerachievementcache.dat
Fini
Code_Load()
VARI_Load()
got 5292 global variables
got 5292 instance variables
got 86 local variables
ID_STRG
Create Window
Create Error Form
Init Graphics
GR_D3D_Init()
OpenGL: version string 2.1 INTEL-14.4.23
OpenGL: vendor string Intel Inc.
OpenGL GLSL: version string 1.20
Extensions:
GL_ARB_color_buffer_float GL_ARB_depth_buffer_float GL_ARB_depth_clamp GL_ARB_depth_texture GL_ARB_draw_buffers GL_ARB_draw_elements_base_vertex GL_ARB_draw_instanced GL_ARB_fragment_program GL_ARB_fragment_program_shadow GL_ARB_fragment_shader GL_ARB_framebuffer_object GL_ARB_framebuffer_sRGB GL_ARB_half_float_pixel GL_ARB_half_float_vertex GL_ARB_instanced_arrays GL_ARB_multisample GL_ARB_multitexture GL_ARB_occlusion_query GL_ARB_pixel_buffer_object GL_ARB_point_parameters GL_ARB_point_sprite GL_ARB_provoking_vertex GL_ARB_seamless_cube_map GL_ARB_shader_objects GL_ARB_shader_texture_lod GL_ARB_shading_language_100 GL_ARB_shadow GL_ARB_sync GL_ARB_texture_border_clamp GL_ARB_texture_compression GL_ARB_texture_compression_rgtc GL_ARB_texture_cube_map GL_ARB_texture_env_add GL_ARB_texture_env_combine GL_ARB_texture_env_crossbar GL_ARB_texture_env_dot3 GL_ARB_texture_float GL_ARB_texture_mirrored_repeat GL_ARB_texture_non_power_of_two GL_ARB_texture_rectangle GL_ARB_texture_rg GL_ARB_transpose_matrix GL_ARB_vertex_array_bgra GL_ARB_vertex_blend GL_ARB_vertex_buffer_object GL_ARB_vertex_program GL_ARB_vertex_shader GL_ARB_window_pos GL_EXT_abgr GL_EXT_bgra GL_EXT_blend_color GL_EXT_blend_equation_separate GL_EXT_blend_func_separate GL_EXT_blend_minmax GL_EXT_blend_subtract GL_EXT_clip_volume_hint GL_EXT_debug_label GL_EXT_debug_marker GL_EXT_draw_buffers2 GL_EXT_draw_range_elements GL_EXT_fog_coord GL_EXT_framebuffer_blit GL_EXT_framebuffer_multisample GL_EXT_framebuffer_multisample_blit_scaled GL_EXT_framebuffer_object GL_EXT_framebuffer_sRGB GL_EXT_geometry_shader4 GL_EXT_gpu_program_parameters GL_EXT_gpu_shader4 GL_EXT_multi_draw_arrays GL_EXT_packed_depth_stencil GL_EXT_packed_float GL_EXT_provoking_vertex GL_EXT_rescale_normal GL_EXT_secondary_color GL_EXT_separate_specular_color GL_EXT_shadow_funcs GL_EXT_stencil_two_side GL_EXT_stencil_wrap GL_EXT_texture_array GL_EXT_texture_compression_dxt1 GL_EXT_texture_compression_s3tc GL_EXT_texture_env_add GL_EXT_texture_filter_anisotropic GL_EXT_texture_integer GL_EXT_texture_lod_bias GL_EXT_texture_rectangle GL_EXT_texture_shared_exponent GL_EXT_texture_sRGB GL_EXT_texture_sRGB_decode GL_EXT_timer_query GL_EXT_transform_feedback GL_EXT_vertex_array_bgra GL_APPLE_aux_depth_stencil GL_APPLE_client_storage GL_APPLE_element_array GL_APPLE_fence GL_APPLE_float_pixels GL_APPLE_flush_buffer_range GL_APPLE_flush_render GL_APPLE_object_purgeable GL_APPLE_packed_pixels GL_APPLE_pixel_buffer GL_APPLE_rgb_422 GL_APPLE_row_bytes GL_APPLE_specular_vector GL_APPLE_texture_range GL_APPLE_transform_hint GL_APPLE_vertex_array_object GL_APPLE_vertex_array_range GL_APPLE_vertex_point_size GL_APPLE_vertex_program_evaluators GL_APPLE_ycbcr_422 GL_ATI_separate_stencil GL_ATI_texture_env_combine3 GL_ATI_texture_float GL_ATI_texture_mirror_once GL_IBM_rasterpos_clip GL_NV_blend_square GL_NV_conditional_render GL_NV_depth_clamp GL_NV_fog_distance GL_NV_light_max_exponent GL_NV_texgen_reflection GL_NV_texture_barrier GL_SGIS_generate_mipmap GL_SGIS_texture_edge_clamp GL_SGIS_texture_lod
Anisotropic filtering supported, max aniso 16
Texture #1 16,16
Texture #2 16,16
Texture #1 16,16
Texture #2 16,16
finished(2)!!
Texture #1 1,1
Texture #2 1,1
finished(2)!!
Background_InitTextures()
Sprite_InitTextures()
IO Init
Process Messages
Splash!
Start Frame
Part Create Textures
Debug Init Remote Interface
VM Init
Create Load Form
Do The Work
LoadGameData()
initialise everything!
Process Chunk: GEN8   352
Process Chunk: OPTN   88
Process Chunk: LANG   24
Process Chunk: EXTN   8
Process Chunk: SOND   1336
Audio_Load()
Process Chunk: AGRP   24
AudioGroup_Load()
Process Chunk: SPRT   7099560
Process Chunk: BGND   190440
Process Chunk: PATH   8
Process Chunk: SCPT   19496
Process Chunk: GLOB   488
Process Chunk: SHDR   1240
Error compiling shader:
ERROR: 0:23: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:23: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'


#version 120
#define LOWPREC
#define lowp
#define mediump
#define highp
#define precision
#define _YY_GLSL_ 1
//
// Simple passthrough vertex shader
//
attribute vec3 in_Position;                  // (x,y,z)
//attribute vec3 in_Normal;                  // (x,y,z)     unused in this shader.
attribute vec4 in_Colour;                    // (r,g,b,a)
attribute vec2 in_TextureCoord;              // (u,v)

varying vec2 v_vTexcoord;
varying vec4 v_vColour;
varying vec3 v_vPosition; //added

void main()
{
vec4 object_space_pos = vec4( in_Position.x, in_Position.y, in_Position.z, 1.0);
gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;

v_vPosition = in_Position;
v_vColour = in_Colour;
v_vTexcoord = in_TextureCoord;
}

Copying error ERROR: 0:23: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:23: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'

Error compiling shader:
ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'


#version 120
#define LOWPREC
#define lowp
#define mediump
#define highp
#define precision
#define _YY_GLSL_ 1
//
// Simple passthrough vertex shader
//
attribute vec3 in_Position;                  // (x,y,z)
//attribute vec3 in_Normal;                  // (x,y,z)     unused in this shader.
attribute vec4 in_Colour;                    // (r,g,b,a)
attribute vec2 in_TextureCoord;              // (u,v)

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main()
{
vec4 object_space_pos = vec4( in_Position.x, in_Position.y, in_Position.z, 1.0);
gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;

v_vColour = in_Colour;
v_vTexcoord = in_TextureCoord;
}

Copying error ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'

Error compiling shader:
ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'


#version 120
#define LOWPREC
#define lowp
#define mediump
#define highp
#define precision
#define _YY_GLSL_ 1
//
// Simple passthrough vertex shader
//
attribute vec3 in_Position;                  // (x,y,z)
//attribute vec3 in_Normal;                  // (x,y,z)     unused in this shader.
attribute vec4 in_Colour;                    // (r,g,b,a)
attribute vec2 in_TextureCoord;              // (u,v)

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main()
{
vec4 object_space_pos = vec4( in_Position.x, in_Position.y, in_Position.z, 1.0);
gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;

v_vColour = in_Colour;
v_vTexcoord = in_TextureCoord;
}

Copying error ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'

Error compiling shader:
ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'


#version 120
#define LOWPREC
#define lowp
#define mediump
#define highp
#define precision
#define _YY_GLSL_ 1
//
// Simple passthrough vertex shader
//
attribute vec3 in_Position;                  // (x,y,z)
//attribute vec3 in_Normal;                  // (x,y,z)     unused in this shader.
attribute vec4 in_Colour;                    // (r,g,b,a)
attribute vec2 in_TextureCoord;              // (u,v)

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main()
{
vec4 object_space_pos = vec4( in_Position.x, in_Position.y, in_Position.z, 1.0);
gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;

v_vColour = in_Colour;
v_vTexcoord = in_TextureCoord;
}

Copying error ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'

Error compiling shader:
ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'


#version 120
#define LOWPREC
#define lowp
#define mediump
#define highp
#define precision
#define _YY_GLSL_ 1
//
// Simple passthrough vertex shader
//
attribute vec3 in_Position;                  // (x,y,z)
//attribute vec3 in_Normal;                  // (x,y,z)     unused in this shader.
attribute vec4 in_Colour;                    // (r,g,b,a)
attribute vec2 in_TextureCoord;              // (u,v)

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main()
{
vec4 object_space_pos = vec4( in_Position.x, in_Position.y, in_Position.z, 1.0);
gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;

v_vColour = in_Colour;
v_vTexcoord = in_TextureCoord;
}

Copying error ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'

Error compiling shader:
ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'


#version 120
#define LOWPREC
#define lowp
#define mediump
#define highp
#define precision
#define _YY_GLSL_ 1
//
// Simple passthrough vertex shader
//
attribute vec3 in_Position;                  // (x,y,z)
//attribute vec3 in_Normal;                  // (x,y,z)     unused in this shader.
attribute vec4 in_Colour;                    // (r,g,b,a)
attribute vec2 in_TextureCoord;              // (u,v)

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main()
{
vec4 object_space_pos = vec4( in_Position.x, in_Position.y, in_Position.z, 1.0);
gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;

v_vColour = in_Colour;
v_vTexcoord = in_TextureCoord;
}

Copying error ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'

Error compiling shader:
ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'


#version 120
#define LOWPREC
#define lowp
#define mediump
#define highp
#define precision
#define _YY_GLSL_ 1
//
// Simple passthrough vertex shader
//
attribute vec3 in_Position;                  // (x,y,z)
//attribute vec3 in_Normal;                  // (x,y,z)     unused in this shader.
attribute vec4 in_Colour;                    // (r,g,b,a)
attribute vec2 in_TextureCoord;              // (u,v)

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main()
{
vec4 object_space_pos = vec4( in_Position.x, in_Position.y, in_Position.z, 1.0);
gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;

v_vColour = in_Colour;
v_vTexcoord = in_TextureCoord;
}

Copying error ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'

Error compiling shader:
ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'


#version 120
#define LOWPREC
#define lowp
#define mediump
#define highp
#define precision
#define _YY_GLSL_ 1
//
// Simple passthrough vertex shader
//
attribute vec3 in_Position;                  // (x,y,z)
//attribute vec3 in_Normal;                  // (x,y,z)     unused in this shader.
attribute vec4 in_Colour;                    // (r,g,b,a)
attribute vec2 in_TextureCoord;              // (u,v)

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main()
{
vec4 object_space_pos = vec4( in_Position.x, in_Position.y, in_Position.z, 1.0);
gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;

v_vColour = in_Colour;
v_vTexcoord = in_TextureCoord;
}

Copying error ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'

Error compiling shader:
ERROR: 0:24: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:24: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'


#version 120
#define LOWPREC
#define lowp
#define mediump
#define highp
#define precision
#define _YY_GLSL_ 1
/*
This shader is the pass-through shader, but it passes through
`in_Position` to `v_position`. Otherwise, identical.
*/

attribute vec3 in_Position;
attribute vec4 in_Colour;
attribute vec2 in_TextureCoord;

varying vec2 v_vTexcoord;
varying vec4 v_vColour;
varying vec3 v_position;

void main() {
v_position = in_Position;
vec4 object_space_pos = vec4(in_Position.x, in_Position.y, in_Position.z, 1.0);
gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;

v_vColour = in_Colour;
v_vTexcoord = in_TextureCoord;
}

Copying error ERROR: 0:24: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:24: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'

Error compiling shader:
ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'


#version 120
#define LOWPREC
#define lowp
#define mediump
#define highp
#define precision
#define _YY_GLSL_ 1
//
// Simple passthrough vertex shader
//
attribute vec3 in_Position;                  // (x,y,z)
//attribute vec3 in_Normal;                  // (x,y,z)     unused in this shader.
attribute vec4 in_Colour;                    // (r,g,b,a)
attribute vec2 in_TextureCoord;              // (u,v)

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main()
{
vec4 object_space_pos = vec4( in_Position.x, in_Position.y, in_Position.z, 1.0);
gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;

v_vColour = in_Colour;
v_vTexcoord = in_TextureCoord;
}

Copying error ERROR: 0:22: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:22: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'

Error compiling shader:
ERROR: 0:17: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:17: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'


#version 120
#define LOWPREC
#define lowp
#define mediump
#define highp
#define precision
#define _YY_GLSL_ 1
attribute vec3 in_Position;     // (x,y,z)
attribute vec4 in_Colour;       // (r,g,b,a)
attribute vec2 in_TextureCoord; // (u,v)

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main() {
vec4 object_space_pos = vec4(in_Position.x, in_Position.y, in_Position.z, 1.0);
gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;

v_vColour = in_Colour;
v_vTexcoord = in_TextureCoord;
}

Copying error ERROR: 0:17: Use of undeclared identifier 'gm_Matrices'
ERROR: 0:17: Use of undeclared identifier 'MATRIX_WORLD_VIEW_PROJECTION'

Process Chunk: FONT   6440
Process Chunk: TMLN   8
Process Chunk: OBJT   47992
Collision Event time(microsecs)=21
Process Chunk: ACRV   8
Process Chunk: SEQN   8
Process Chunk: TAGS   744
Process Chunk: ROOM   5374136
Process Chunk: DAFL   8
Process Chunk: EMBI   120
Process Chunk: TPAG   63352
Process Chunk: TGIN   3448
Process Chunk: CODE   757096
Process Chunk: VARI   82632
Process Chunk: FUNC   41784
Process Chunk: STRG   292936
Process Chunk: TXTR   7785704
Process Chunk: AUDO   65077300
Audio_WAVs()
PrepareGame()
Extension_Prepare()
Code_Constant_Prepare()
Script_Prepare()
TimeLine_Prepare()
Object_Prepare()
Preparing 105 objects:
Room_Prepare()
Sound_Prepare()
InitGraphics()
Finished PrepareGame()
Run_Start
[Sun Sep 27 15:05:58 2020] TRACE gml_Object_Game_Create_0:46 -- beginning game setup...
[Sun Sep 27 15:05:58 2020] WARN gml_Script_deserialize_Configuration_gml_GlobalScript_Configuration:35 -- we still check for mvc when we can safely switch over to version soon.
[Sun Sep 27 15:05:58 2020] TRACE gml_Object_Game_Create_0:256 --   attempted to load save.json
[Sun Sep 27 15:05:58 2020] TRACE gml_Script_Camera:380 -- Creating new Mistria GUI! [Reason: Bootup]
[Sun Sep 27 15:05:58 2020] TRACE gml_Script_Camera:383 -- Creating new Window GUI! [Reason: Bootup]
[Sun Sep 27 15:05:58 2020] TRACE gml_Script_set_view_size_Camera_gml_GlobalScript_CameraClass:110 -- Creating new Mistria GUI! [Reason: View Resize]
pathfinding grid initialized at (32.00, 32.00),  with keysize of 8
[Sun Sep 27 15:05:58 2020] WARN gml_Object_Game_Create_0:1040 -- Gabe is doing some graphic stuff here that he doesn't know where else to put...
texture_prefetch(): Texture group Grass not found
Total memory used = 138077120(0x083ae3c0) bytes
**********************************.
Entering main loop.
**********************************.
[Sun Sep 27 15:05:59 2020] TRACE gml_Script_play_track_Boombox_gml_GlobalScript_Boombox:123 -- Playing new music track
[Sun Sep 27 15:05:59 2020] TRACE gml_Script_anon_play_track_Boombox_gml_GlobalScript_Boombox_3181_play_track_Boombox_gml_GlobalScript_Boombox:136 -- Set music state to FadeIn
Texture #3 4096,4096
Texture #3 4096,4096
Texture #3 2048,2048
[Sun Sep 27 15:06:00 2020] TRACE gml_Script_anon_play_track_Boombox_gml_GlobalScript_Boombox_3596_play_track_Boombox_gml_GlobalScript_Boombox:141 -- Set music state to On
Igor complete.


*/
