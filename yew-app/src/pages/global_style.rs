use stylist::StyleSource;

pub fn global_style () -> StyleSource<'static> {
    let global_style = stylist::css!(r#"
    html {
        font-size: 62.5%;
      }
      
      html.sr .load-hidden {
        visibility: hidden;
      }
    "# );
    
    global_style
}