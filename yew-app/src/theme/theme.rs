


#[derive(Clone, Debug, PartialEq)]

struct ColorTheme {
    highlight_color : String,
    main_color: String,
    contrast_color: String,

}
#[derive(Clone, PartialEq)]
pub struct Theme {
    dark_theme: ColorTheme,
    light_theme: ColorTheme,
}



pub static COLOR_THEME:Theme = { Theme {
    dark_theme:{ ColorTheme {
        highlight_color:"#0D1E3C".to_owned(),
        main_color:"#002C69".to_owned(),
        contrast_color:"#584D80".to_owned()
    } },
    light_theme:{ColorTheme{
        highlight_color:"#156064".to_owned(),
        main_color:"#00C49A".to_owned(),
        contrast_color:"#F8E16C".to_owned()
    }}
} };