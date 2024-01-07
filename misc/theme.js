function hexToRGB(hex) {
  const [r, g, b] = [hex.slice(1, 3), hex.slice(3, 5), hex.slice(5, 7)]
    .map((hex) => parseInt(hex, 16) / 255)
    .map((num) => num.toFixed(8));

  return `Color::rgb(${r}, ${g}, ${b})`;
}

// paste a theme here from either:
// - https://github.com/saadeghi/daisyui/blob/master/src/theming/themes.js
// - https://daisyui.com/theme-generator/
// and then run `node ./misc/theme.js`
const themes = {
  dracula: {
    primary: "#ff79c6",
    secondary: "#bd93f9",
    accent: "#ffb86c",
    neutral: "#414558",
    "base-100": "#282a36",
    info: "#8be9fd",
    success: "#50fa7b",
    warning: "#f1fa8c",
    error: "#ff5555",
  },
};

for (const themeName of Object.keys(themes)) {
  const mappedColors = Object.entries(themes[themeName])
    .map(([key, value]) => [key.replace("-", ""), hexToRGB(value)])
    .map(([key, value]) => `\t${key}: ${value},\n`)
    .join("");

  const output = `const ${themeName.toUpperCase()}: Theme = Theme {\n${mappedColors}};`;
  console.log(output);
}
