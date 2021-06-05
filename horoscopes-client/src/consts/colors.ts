type Color = string;

const ColorPalette = {
  black: '#000000',
  ebony: '#1C1C1C',
  midnight: '#3B3B3B',
  white: '#FBFBFB',
  ivory: '#EFEFEF',
};

type ColorKeys =
  | 'textPrimary'
  | 'textSecondary'
  | 'backgroundPrimary'
  | 'backgroundSecondary'
  | 'backgroundTertiary'
  | 'shadowPrimary';

type ColorSet = { [Key in ColorKeys]: Color };

const basicColorSet: ColorSet = {
  textPrimary: ColorPalette.ebony,
  textSecondary: ColorPalette.midnight,
  backgroundPrimary: ColorPalette.ivory,
  backgroundSecondary: ColorPalette.white,
  backgroundTertiary: ColorPalette.midnight,
  shadowPrimary: ColorPalette.midnight + '11',
};

export { basicColorSet };
