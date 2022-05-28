use stylist::StyleSource;


pub fn hero_style () -> StyleSource<'static> {
  let hero_style = stylist::css!(r#"


  #hero {
    min-height: 100vh;
    height: 100vh;
    display: flex;
    flex-direction:column;
    align-items: start;
    justify-content: start;
    border-bottom: 0px;
    font-weight: 400;
    color: #272341;
    padding: 0rem 4rem; 
    margin-bottom: 0;
   
    z-index: -1;
 
  }


  #hero .hero-title {
    font-size: 6rem;
    font-weight: 700;
    margin-bottom: 1rem;
    text-align: left;
  }

  #hero .hero-cta-div{
    display: flex;
  }




  #hero .hero-cta-div button {
    margin-left: 1rem;
    border-radius: 5px;
    font-size: 2.4rem;
    border: none;
    background-color: rgb(88, 164, 255);
  }


  #hero .text-typing {
    margin:0px;
    white-space:nowrap;
    overflow:hidden;
    animation:typing 2s steps(19,end) forwards,
              blink 0.2s 10;
  }

  #hero .text-typing2 {
    visibility: hidden;
    margin:0px;
    white-space:nowrap;
    overflow:hidden;
    animation:typing 2s steps(15,end) forwards,
      blink 0.2s infinite, show 1s infinite;
    animation-delay: 2s;
  }


  .text-color-main{
    color: blue;
    width:fit-content; 
  }

  @keyframes fade-in {
    100% {
      opacity: 1;
      filter: blur(0);
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












