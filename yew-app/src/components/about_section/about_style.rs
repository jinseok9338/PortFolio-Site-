use stylist::StyleSource;

pub fn about_style() -> StyleSource<'static> {
    let about_style = stylist::css!(
        r#"

  #about {
    background-color: #02aab0;
    background-image: linear-gradient(135deg, #02aab0 0%, #00cdac 100%);
    color: #fff;
    height: 100%;
    border-top: 0px;
    -webkit-clip-path: polygon(0 0, 100% 0, 100% 80%, 0 100%);
    clip-path: polygon(0 0, 100% 0, 100% 80%, 0 100%);
    padding-bottom: 10%;
    display:flex;
    justify-content: center;
 }

  #about .about-wrapper__image {
    display: flex;
    height: 100%;
    align-items: center;
    justify-content: center;
 }

 #about .about-wrapper__info {
  display: flex;
  height: 100%;
  justify-content: center;
  flex-direction: column;
}

#about .about-wrapper__info-text {
  text-align: left;
}

#about .about-wrapper__info-text--important {
  background: #00c8a8;
  display: inline-block;
  font-style: italic;
  padding: 0 0.3rem;
  margin: 0.3rem 0;
  line-height: 1.6;
}

 @media (max-width: 75em) {
  #about {
    height: 100%;
    -webkit-clip-path: none;
    clip-path: none;
 }
}
@media (max-width: 37.5em) {
  #about .about-wrapper {
    padding-bottom: 5rem;
 }
}

  @media (max-width: 75em) {
    #about .about-wrapper__image {
      height: 100%;
   }
 }
  @media (max-width: 48em) {
    #about .about-wrapper__image {
      padding-bottom: 6.4rem;
   }
 }

  @media (max-width: 48em) {
    #about .about-wrapper__info {
      align-items: center;
   }
 }

  @media (max-width: 56.25em) {
    #about .about-wrapper__info-text {
      text-align: left;
   }
 }
  @media (max-width: 48em) {
    #about .about-wrapper__info-text {
      text-align: center;
   }
 }

  @media (max-width: 37.5em) {
    #about .about-wrapper__info-text--important {
      display: inline;
      margin: 0;
      padding: 0;
      background: transparent;
      font-style: normal;
   }
 }

  "#
    );
    return about_style;
}
