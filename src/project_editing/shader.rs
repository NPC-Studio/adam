use std::process::ExitCode;

use yy_boss::{ShaderFile, YypBoss};
use yy_typings::{CommonData, Shader};

use crate::input::ShaderEditRequest;

const DEFAULT_VTX_SHADER: &str = "\
attribute vec3 in_Position;
attribute vec4 in_Colour;
attribute vec2 in_TextureCoord;

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main()  {
    vec4 object_space_pos = vec4(in_Position.x, in_Position.y, in_Position.z, 1.0);
    gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;

    v_vColour = in_Colour;
    v_vTexcoord = in_TextureCoord;
}

";

const DEFAULT_FRAG_SHADER: &str = "\
varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main() {
    gl_FragColor = v_vColour * texture2D(gm_BaseTexture, v_vTexcoord);
}

";

pub fn add_shader(shader: ShaderEditRequest) -> ExitCode {
    let Some(mut yyp_boss) = super::create_yyp_boss(YypBoss::without_resources) else {
        return ExitCode::FAILURE;
    };
    yyp_boss
        .quick_name()
        .expect("bad yyp entry -- couldn't add.");

    let parent = super::find_vfs_path(&yyp_boss, &shader.folder).unwrap_or_else(|| {
        adam_warning!(
            "Folder `{}` did not exist. Placing in Project Root.",
            shader.folder
        );

        yyp_boss.project_metadata().root_file
    });

    if let Err(e) = yyp_boss.add_resource(
        Shader {
            common_data: CommonData::new(shader.name.clone()),
            parent,
            shader_type: match shader.shader_type {
                crate::input::ShaderEditShaderType::GlslEs => yy_typings::ShaderType::GlslEs,
                crate::input::ShaderEditShaderType::Glsl => yy_typings::ShaderType::Glsl,
                crate::input::ShaderEditShaderType::Hlsl => yy_typings::ShaderType::Hlsl,
            },
        },
        ShaderFile {
            vertex: DEFAULT_VTX_SHADER.to_string(),
            pixel: DEFAULT_FRAG_SHADER.to_string(),
        },
    ) {
        println!("{}: {}", console::style("error").bright().red(), e);
        return ExitCode::FAILURE;
    }
    if let Err(e) = yyp_boss.serialize() {
        println!(
            "{}: could not serialize {}",
            console::style("error").bright().red(),
            e
        );
        return ExitCode::FAILURE;
    }

    println!(
        "{}: ./shaders/{SHADER_NAME}/{SHADER_NAME}.vsh, ./shaders/{SHADER_NAME}/{SHADER_NAME}.fsh",
        console::style("success").green().bright(),
        SHADER_NAME = shader.name
    );

    ExitCode::SUCCESS
}
