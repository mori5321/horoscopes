type Color = string;

const ColorPalette = {
  black: '#000000',
  ebony: '#1C1C1C',
  midnight: '#3B3B3B',
  darkGray: '#555555',
  white: '#FBFBFB',
  ivory: '#EFEFEF',
  navy: '#1E1242',
  purple: '#7F21C9',
  pinkPurple: '#C9217B'
};

type ColorKeys =
  | 'textPrimary'
  | 'textSecondary'
  | 'textTertiary'
  | 'textQuaternary'
  | 'textQuinary'
  | 'backgroundPrimary'
  | 'backgroundSecondary'
  | 'backgroundTertiary'
  | 'backgroundQuaternary'
  | 'shadowPrimary'
  | 'borderPrimary'
  | 'accentPrimary'
  | 'accentSecondary'

type ColorSet = { [Key in ColorKeys]: Color };

const basicColorSet: ColorSet = {
  textPrimary: ColorPalette.ebony,
  textSecondary: ColorPalette.midnight,
  textTertiary: ColorPalette.darkGray,
  textQuaternary: ColorPalette.white,
  textQuinary: ColorPalette.white,
  backgroundPrimary: ColorPalette.ivory,
  backgroundSecondary: ColorPalette.white,
  backgroundTertiary: ColorPalette.midnight,
  backgroundQuaternary: ColorPalette.navy,
  shadowPrimary: ColorPalette.midnight + '11',
  borderPrimary: ColorPalette.midnight,
  accentPrimary: ColorPalette.purple,
  accentSecondary: ColorPalette.pinkPurple,
};

export { basicColorSet };
