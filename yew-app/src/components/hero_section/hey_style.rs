use stylist::StyleSource;


pub fn hey_style () -> StyleSource<'static> {
  let hey_style = stylist::css!(r#"
  
  *, *::before, *::after {
    box-sizing: border-box;
}
:root {
    --color-primary: #f6aca2;
    --color-secondary: #f49b90;
    --color-tertiary: #f28b7d;
    --color-quaternary: #f07a6a;
    --color-quinary: #ee6352;
   /* --color-primary: #5192ED;
    --color-secondary: #69A1F0;
    --color-tertiary: #7EAEF2;
    --color-quaternary: #90BAF5;
    --color-quinary: #A2C4F5;
    */
}

.content {
    display: flex;
    align-content: center;
    justify-content: start;
}
.text_shadows {
    text-shadow: 3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary);
    font-family: bungee, sans-serif;
    font-weight: 800;
    text-transform: uppercase;
    font-size: calc(22rem + 5vw);
    text-align: center;
    margin: 0;
    margin-left:3rem;
    color: var(--color-primary);
    animation: shadows 1.2s ease-in infinite, move 1.2s ease-in infinite;
    letter-spacing: 0.4rem;
}
@keyframes shadows {
    0% {
        text-shadow: none;
   }
    10% {
        text-shadow: 3px 3px 0 var(--color-secondary);
   }
    20% {
        text-shadow: 3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary);
   }
    30% {
        text-shadow: 3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary);
   }
    40% {
        text-shadow: 3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary);
   }
    50% {
        text-shadow: 3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary);
   }
    60% {
        text-shadow: 3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary);
   }
    70% {
        text-shadow: 3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary);
   }
    80% {
        text-shadow: 3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary);
   }
    90% {
        text-shadow: 3px 3px 0 var(--color-secondary);
   }
    100% {
        text-shadow: none;
   }
}
@keyframes move {
    0% {
        transform: translate(0px, 0px);
   }
    40% {
        transform: translate(-12px, -12px);
   }
    50% {
        transform: translate(-12px, -12px);
   }
    60% {
        transform: translate(-12px, -12px);
   }
    100% {
        transform: translate(0px, 0px);
   }
}



  "#);

  return hey_style
}