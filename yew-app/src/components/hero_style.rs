use stylist::StyleSource;


pub fn hero_style () -> StyleSource<'static> {
  let hero_style = stylist::css!(r#"


  #hero {
    min-height: 100vh;
    height: 100vh;
    display: flex;
    align-items: center;
    border-bottom: 0px;
    background: #fff;
    font-weight: 400;
    color: #272341;
    padding: 0rem 5.6rem;
    margin-bottom: 0;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
    z-index: -1;
    /* background: url("/src/assets/[your-image].png");
    background-position: center;
    background-size: cover; */
  }
  @media (max-width: 37.5em) {
    #hero {
      padding: 0rem 1.6rem;
    }
  }
  #hero .hero-title {
    font-size: 5.6rem;
    font-weight: 700;
    margin-bottom: 3.2rem;
    text-align: left;
  }
  @media (max-width: 75em) {
    #hero .hero-title {
      font-size: 4rem;
    }
  }
  @media (max-width: 56.25em) {
    #hero .hero-title {
      font-size: 3.6rem;
      text-align: center;
    }
  }
  @media (max-width: 37.5em) {
    #hero .hero-title {
      font-size: 3.5rem;
      line-height: 1.5;
    }
  }
  @media (max-width: 20em) {
    #hero .hero-title {
      font-size: 2.8rem;
    }
  }
  #hero .hero-cta {
    display: flex;
  }
  @media (max-width: 56.25em) {
    #hero .hero-cta {
      justify-content: center;
    }
  }
  #hero .hero-cta a {
    font-size: 2.4rem;
  }
  @media (max-width: 37.5em) {
    #hero .hero-cta a {
      font-size: 2rem;
    }
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
  
  .section-title {
    margin: 0px;
    margin-bottom: 4.5rem;
    font-size: 4rem;
    font-weight: bold;
    text-transform: uppercase;
  }

  "#);

  return hero_style
}












