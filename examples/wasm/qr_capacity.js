// QR capacity table for ALPHANUMERIC mode (versions 1..40)
// Source: Based on standard QR Code capacity tables (ISO/IEC 18004) widely circulated
// Reference (one of many summaries): https://www.qrcode.com/en/about/version.html
// Arrays are DATA BIT capacities (total capacity minus overhead), per error correction level.

export const CAP_ALNUM_BITS = {
  L: [152,272,440,640,864,1088,1248,1552,1856,2192,2592,2960,3424,3688,4184,4712,5176,5768,6360,6888,7456,8048,8752,9392,10208,10960,11744,12248,13048,13880,14744,15640,16568,17528,18448,19472,20528,21616,22496,23648],
  M: [128,224,352,512,688,864,992,1232,1456,1728,2032,2320,2672,2920,3320,3624,4056,4504,5016,5352,5712,6256,6880,7312,8000,8496,9024,9544,10136,10984,11640,12328,13048,13800,14496,15312,15936,16816,17728,18672],
  Q: [104,176,272,384,496,608,704,880,1056,1232,1440,1648,1952,2088,2360,2600,2936,3176,3560,3880,4096,4544,4912,5312,5744,6032,6464,6968,7288,7880,8264,8920,9368,9848,10288,10832,11408,12016,12656,13328],
  H: [72,128,208,288,368,480,528,688,800,976,1120,1264,1440,1576,1784,2024,2264,2504,2728,3080,3248,3536,3712,4112,4304,4768,5024,5288,5608,5960,6344,6760,7208,7688,7888,8432,8768,9136,9776,10208]
};

// Mapping to the library's error correction level constants
export const QR_EC = { L: 1, M: 0, Q: 3, H: 2 }; // library uses: 0=M,1=L,2=H,3=Q

// Calculate bits needed for alphanumeric data
// Mode indicator: 4 bits
// Character count indicator: 9 bits (v1-9), 11 bits (v10-26), 13 bits (v27-40)
// Data: pairs of chars = 11 bits each, single char = 6 bits
export function calculateAlnumBits(len, version) {
  const modeIndicator = 4;
  const charCountIndicator = version <= 9 ? 9 : (version <= 26 ? 11 : 13);
  const pairs = Math.floor(len / 2);
  const single = len % 2;
  const dataBits = pairs * 11 + single * 6;
  return modeIndicator + charCountIndicator + dataBits;
}

export function minimalVersionForAlnum(len, ecLevel) {
  const arr = CAP_ALNUM_BITS[ecLevel];
  if (!arr) return null;

  for (let i = 0; i < arr.length; i++) {
    const version = i + 1;
    const requiredBits = calculateAlnumBits(len, version);
    if (requiredBits <= arr[i]) {
      return version;
    }
  }
  return null; // exceeds version 40
}
