use globwalk::glob;
use ramhorns::Template;

use crate::templating::ChunkTemplate;

const GLOB_LUA_FILES: &'static str = "test/**/*.{lua,luau}";
const LUA_RUNTIME: &'static str = include_str!("./runtime.lua");

// Runtime Funcs
const CREATE_CHUNK_LUA: &'static str = r#"
-- module: {{ path }}
__moonpack__.chunks["{{ name }}"] = function(script)
    {{ code }}
end
"#;

/// Get all the lua files in the project
/// # Returns
/// A vector of strings containing the paths to the lua files
pub fn get_all_lua_files_contents() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let walker = glob(GLOB_LUA_FILES).unwrap();
    let mut lua_files: Vec<String> = Vec::new();
    for entry in walker {
        let entry = entry?;
        println!("{}", entry.path().display());

        let file_contents = std::fs::read_to_string(entry.path())?;
        lua_files.push(file_contents);
    }

    return Ok(lua_files);
}

/// Entry point for the bundler
pub fn bundle() -> Result<(), Box<dyn std::error::Error>> {
    let files = get_all_lua_files_contents()?;
    let bundle_output_list = vec![LUA_RUNTIME];

    let tpl = Template::new(CREATE_CHUNK_LUA).unwrap();
    let re = tpl.render(&ChunkTemplate {
        path: "runtime".to_string(),
        name: "runtime".to_string(),
        code: "print('hello')".to_string(),
    });

    println!("{}", re);

    // let hmap = hashmap!("path" => "runtime", "name" => "runtime", "code" => "print('hello')");
    // println!("{:?}", hmap);
    // template(CREATE_CHUNK_LUA.to_string(), hmap);

    return Ok(());
}
