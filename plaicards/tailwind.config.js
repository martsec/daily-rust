/** @type {import('tailwindcss').Config} */
const plugin = require('tailwindcss/plugin')
module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  // Need to force some classes since we use them parametrized
  safelist: [
    'text-2xl',
    'text-3xl',
    'text-color-green',
    {
      pattern: /bg-card-*/,
    },
    {
      pattern: /text-color-(orange|green|blue|yellow)/,
    },
    {
      pattern: /bg-(orange|green|blue|yellow)-(100|500|600|900)/,
      variants: ['lg', 'hover', 'focus', 'lg:hover'],
    },
    {
      pattern: /border-(orange|green|blue|yellow)/,
    },
    {
      pattern: /border-(orange|green|blue|yellow)-(100|900)/,
      variants: ['lg', 'hover', 'focus', 'lg:hover'],
    },
  ],
  theme: {
    extend: {
      fontSize: {
        "13xl": "12rem",
      },
      backgroundImage: {
        'texture-paper': "url('../img/texture-paper.jpg')",
        'card-back': "url('../img/card_back.png')",
        'card-special': "url('../img/card_special.png')",
        'card-usecase': "url('../img/card_useCase.png')",
        'card-marketevent': "url('../img/card_marketEvent.png')",
        'card-buzzword': "url('../img/card_buzzword.png')",
        'card-adversary': "url('../img/card_adversary.png')",
        'card-antitrust': "url('../img/card_antitrust.png')",
        'card-dotcom': "url('../img/card_dotcom.png')",
        'card-hr': "url('../img/card_hr.png')",
        'card-moredata': "url('../img/card_more_data.png')",
        'card-aiarmy': "url('../img/card_ai_army.png')",
        'card-toxic': "url('../img/card_toxic_manager.png')",
        'card-daily': "url('../img/card_daily.png')", 
        'card-criminals': "url('../img/card_criminals_faces.png')", 
        'card-winart': "url('../img/card_winart.png')", 
      },
      keyframes: {
          "fade-in": {
            "0%": {"opacity": "0" },
            "100%": { "opacity": "1"}
          },
          "fade-slide-in-right": {
            "0%": {opacity: "0", transform: "translateX(75%)"},
            "100%": {opacity: "1", transform: "translateX(0)"},
          },
          "fade-slide-in-left": {
            "0%": {opacity: "0", transform: "translateX(-75%)"},
            "100%": {opacity: "1", transform: "translateX(0)"},
          },
          slideIn: {
            "0%": { 
              opacity: 1, 
              transform: "translateX(2rem)" 
            },
            "100%": { 
              opacity: 1, 
              transform: "translateX(0)" 
            }
          },
          wiggle: {
          '0%, 100%': { transform: 'rotate(-3deg)' },
          '50%': { transform: 'rotate(3deg)' },
        },
        animatedgradient: {
          '0%': { backgroundPosition: '0% 50%' },
          '50%': { backgroundPosition: '100% 50%' },
          '100%': { backgroundPosition: '0% 50%' },
        },
        slide: {
          "0%": { transform: "translateY(100%)", opacity: 0.1 },
          "15%": { transform: "translateY(0)", opacity: 1 },
          "30%": { transform: "translateY(0)", opacity: 1 },
          "45%": { transform: "translateY(-100%)", opacity: 1 },
          "100%": { transform: "translateY(-100%)", opacity: 0.1 },
        },
        'scroll-right': {
          "0%": { transform: 'translateX(0)' },
          "50%": { transform: 'translateX(-50%)' },
          "100%": { transform: 'translateX(0)' },
        },
        'scroll-left': {
          "0%": { transform: 'translateX(-50%)' },
          "50%": { transform: 'translateX(0)' },
          "100%": { transform: 'translateX(-50%)' },
        },
        'infinite-scroll': {
            from: { transform: 'translateX(0)' },
            to: { transform: 'translateX(-100%)' },
        },
      },
      backgroundSize: {
        '300%': '300%',
      },
      animation: {
        "fade-in": "fade-in 0.6s ease-in",
        "fade-slide-in-left": "fade-slide-in-left 1s ease-in",
        "fade-slide-in-right": "fade-slide-in-right 1s ease-in",
        slideIn: "slideIn 1s ease-in",
        wiggle: 'wiggle 1s ease-in-out infinite',
        // https://www.andrealves.dev/blog/how-to-make-an-animated-gradient-text-with-tailwindcss/
        gradient: 'animatedgradient 6s ease infinite alternate',
        // https://www.ibelick.com/blog/create-text-sliding-effect-with-tailwind-css
        slide: "slide 9s linear infinite",
        'infinite-scroll': 'infinite-scroll 100s linear infinite',
        scrollRight: "scroll-right 90s linear infinite",
        scrollLeft: "scroll-left 80s linear infinite",
      },
      colors: {
        green: {
          DEFAULT: '#287d3b',
          50: '#f2fbf4',
          100: '#e1f7e5',
          200: '#c3efcc',
          300: '#95e0a5',
          400: '#5fc976',
          500: '#39ae52',
          600: '#2a8f40',
          700: '#287d3b',
          800: '#215a2e',
          900: '#1d4a28',
          950: '#0b2813',
        },
        orange: {
          DEFAULT: '#b16227',
          50: '#fcf7ee',
          100: '#f5ead0',
          200: '#ead39d',
          300: '#e0b769',
          400: '#d8a047',
          500: '#cf8331',
          600: '#b16227',
          700: '#984b25',
          800: '#7d3b23',
          900: '#673220',
          950: '#3a180e',
        },
        blue: {
          DEFAULT: '#4aa8b8',
          50: '#f1fafa',
          100: '#dbf0f2',
          200: '#bae1e7',
          300: '#8bccd5',
          400: '#4aa8b8' /* default */,
          500: '#3991a1',
          600: '#327688',
          700: '#2e6170',
          800: '#2d515d',
          900: '#294450',
          950: '#172d35',
        },
        yellow: {
          DEFAULT: '#feb511',
          50: '#fffcea',
          100: '#fff8c5',
          200: '#fff186',
          300: '#ffe346',
          400: '#ffd11c',
          500: '#feb511',
          600: '#e18600',
          700: '#bb5e02',
          800: '#974809',
          900: '#7c3b0b',
          950: '#481e00',
        },
        gray: {
          DEFAULT: '#808080',
          50: '#f6f6f6',
          100: '#e7e7e7',
          200: '#d1d1d1',
          300: '#b0b0b0',
          400: '#808080',
          500: '#6d6d6d',
          600: '#5d5d5d',
          700: '#4f4f4f',
          800: '#454545',
          900: '#3d3d3d',
          950: '#262626',
        },
        'dove-gray': {
          DEFAULT: '#706F6F',
          50: '#f6f5f5',
          100: '#e7e6e6',
          200: '#d1d0d0',
          300: '#b0b0b0',
          400: '#888888',
          500: '#706f6f',
          600: '#5d5d5d',
          700: '#4f4f4f',
          800: '#454545',
          900: '#3d3c3c',
          950: '#262626',
        },
        'gray-illustration': {
          DEFAULT: '#3e3e3d',
          50: '#f6f6f5',
          100: '#e7e7e6',
          200: '#d1d1d0',
          300: '#b1b0af',
          400: '#898887',
          500: '#6e6d6c',
          600: '#5e5d5c',
          700: '#50504e',
          800: '#464644',
          900: '#3e3e3d',
          950: '#262626',
        },
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms')
  ],
}
