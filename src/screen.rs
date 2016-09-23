/// The Gameboy screen is a 160x144 pixel display with 20x18 tiles.
/// The screen size is 2.6 inches which is about 6.6 centimeters.

// TODO: sort this info as it probably will have nothing to do with the screen's
// emulation.
// --
// The maximum number of sprites are 40.
// The maximum number of sprites per line is 10.
// The maximum sprite size is
// The horizontal sync of the display is 9.198 KHz
// The vertical sync of the display is 59.73 Hz

// The main GB screen buffer (background) consists of 256x256 pixels or 32x32
// tiles of 8x8 pixels each. only 160x144 pixels can be displayed on screen.
// Registers `SCROLLX` and `SCROLLY` hold the coordinates of the background to be
// displayed in the upper left corner of the screen. Background wraps around
// the screen; if one side goes off, it'll appear on the other side.

// An area of VRAM known as the Background Tile Map contains the number of
// tiles to be displayed; organized as 32 rows of 32 bytes. Each byte contains
// a number of a tile to be displayed. Tile patterns are taken from the Tile
// Data Table located either at 0x8000 - 0x8FFF or 0x8800 - 0x97FF. In the first
// case patterns are numbered with unsigned numbers from 0 to 255. in the second
// case, the patterns have signed numbers from -128 to 127. The Tile Data Table
// address can be selected via the `LCDC` register.

// Besides a background, there is also a 'window' overlaying the background. The
// window is not scroll-able and always starts from the upper left corner.
// The location of the window can be adjusted by the `WNDPOSX` and `WNDPOSY`
// registers. Screen coordinates for the top left corner of a window are
// (WNDPOSX - 7, WNDPOSY). The tile numbers are stored in the Tile Data Table.
// None of the window tiles are ever transparent and both the background and the
// window share the same Tile Data Table. The background and window can be
// enabled or disabled separately via bits in the `LCDC` register.
