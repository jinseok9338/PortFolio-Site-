use stylist::StyleSource;


pub fn hero_style () -> StyleSource<'static> {
  let hero_style = stylist::css!(r#"


  #hero {
    min-height: 100vh;
    height: 100vh;
    display: flex;
    flex-direction:column;
    align-items: start;
    jusfity-content: center;
    border-bottom: 0px;
    background: #fff;
    font-weight: 400;
    color: #272341;
    padding: 0rem 5.6rem;
    margin-bottom: 0;
   
    z-index: -1;
 
  }


  #hero .hero-title {
    font-size: 6rem;
    font-weight: 700;
    margin-bottom: 1rem;
    text-align: left;
  }

  #hero .hero-cta {
    display: flex;
    animation: from_above 1s;
    animation-delay: 10s;
    animation-fill-mode: forwards;
  }

  #hero .hero-cta a {
    font-size: 2.4rem;
  }


  #hero .text-typing {
    margin:0px;
    white-space:nowrap;
    overflow:hidden;
    animation:typing 4s steps(20,end) forwards,
              blink 5s ;
  }

  #hero .text-typing2 {
    visibility: hidden;
    margin:0px;
    white-space:nowrap;
    overflow:hidden;
    animation:typing 4s steps(22,end) forwards,
              blink 5s, show 1s infinite;
    animation-delay: 5s
  }


  .text-color-main{
    color: blue;
    width:fit-content; 
  }

  @keyframes from_above {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes show {
    0% { visibility:visible }
    100% { visible:visible }
  }

  @keyframes typing {
    0% { width:0% }
    100% { width:100% }
  }

  @keyframes blink {
    0%,100% {
      border-right:2px solid transparent;
    }
    50% {
      border-right:2px solid #222;
    }
  }


  /* these are for media query */

  @media (max-width: 37.5em) {
    #hero {
      padding: 0rem 1.6rem;
    }
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

  @media (max-width: 56.25em) {
    #hero .hero-cta {
      justify-content: center;
    }
  }

  @media (max-width: 37.5em) {
    #hero .hero-cta a {
      font-size: 2rem;
    }
  }
 
  "#);

  return hero_style
}












