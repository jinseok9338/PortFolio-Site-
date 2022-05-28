use stylist::StyleSource;

pub fn global_style () -> StyleSource<'static> {
    let global_style = stylist::css!(r#"
    html {
        font-size: 62.5%;
        background-color:#33CCFF
      }
      
      html.sr .load-hidden {
        visibility: hidden;
      }

      h1 {
        font-weight: 700;
      }
      
      p,
      a {
        font-family: "Montserrat", sans-serif;
        font-size: 1.6rem;
      }
      
      a,
      a:link,
      a:hover,
      a:visited,
      a:active {
        text-decoration: none;
      }
      
      a:hover {
        transition: all 0.3s ease-in-out;
      }

      .dummy {
        width: 100vw;
        height: 13vh;
      }

    "# );
    
    global_style
}