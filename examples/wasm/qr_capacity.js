// QR capacity table for ALPHANUMERIC mode (versions 1..40)
// Source: ISO/IEC 18004 standard
// Reference: https://www.qrcode.com/en/about/version.html

export const CAP_ALNUM_CHARS = {
  L: [25,47,77,114,154,195,224,279,335,395,468,535,619,667,758,854,938,1046,1153,1249,1352,1460,1588,1704,1853,1990,2132,2223,2369,2520,2677,2840,3009,3183,3351,3537,3729,3927,4087,4296],
  M: [20,38,61,90,122,154,178,221,262,311,366,419,483,528,600,656,734,816,909,970,1035,1134,1248,1326,1451,1542,1637,1732,1839,1994,2113,2238,2369,2506,2632,2780,2894,3054,3220,3391],
  Q: [16,29,47,67,87,108,125,157,189,221,259,296,352,376,426,470,531,574,644,702,742,823,890,963,1041,1094,1172,1263,1322,1429,1499,1618,1700,1787,1867,1966,2071,2181,2298,2420],
  H: [10,20,35,50,64,84,93,122,143,174,200,227,259,283,321,365,408,452,493,557,587,640,672,744,779,864,910,958,1016,1080,1150,1226,1307,1394,1431,1530,1591,1658,1774,1852]
};

// Calculate QR alphanumeric data bits (for display purposes)
// Mode: 4 bits, Count: 9/11/13 bits, Data: 11 bits per pair + 6 bits per single char
export function calculateAlnumBits(len, version) {
  const modeIndicator = 4;
  const charCountIndicator = version <= 9 ? 9 : (version <= 26 ? 11 : 13);
  const pairs = Math.floor(len / 2);
  const single = len % 2;
  const dataBits = pairs * 11 + single * 6;
  return modeIndicator + charCountIndicator + dataBits;
}

export function minimalVersionForAlnum(len, ecLevel) {
  const arr = CAP_ALNUM_CHARS[ecLevel];
  if (!arr) return null;

  for (let i = 0; i < arr.length; i++) {
    if (len <= arr[i]) {
      return i + 1; // versions start at 1
    }
  }
  return null; // exceeds version 40
}
