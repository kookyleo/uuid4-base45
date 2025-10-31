// QR capacity table for ALPHANUMERIC mode (versions 1..40)
// Source: Based on standard QR Code capacity tables (ISO/IEC 18004) widely circulated
// Reference (one of many summaries): https://www.qrcode.com/en/about/version.html
// Arrays are capacities for the alphanumeric mode, per error correction level.

export const CAP_ALNUM = {
  L: [25,47,77,114,154,195,224,279,335,395,468,535,619,667,758,854,938,1046,1153,1249,1352,1460,1588,1704,1853,1990,2132,2223,2369,2520,2677,2840,3009,3183,3351,3537,3729,3927,4087,4296],
  M: [20,38,61,90,122,154,178,221,262,311,366,419,483,528,600,656,734,816,909,970,1035,1134,1248,1326,1451,1542,1637,1732,1839,1994,2113,2238,2369,2506,2632,2780,2894,3054,3220,3391],
  Q: [16,29,47,67,87,108,125,157,189,221,259,296,352,376,426,470,531,574,644,702,742,823,890,963,1041,1094,1172,1263,1322,1429,1499,1618,1700,1787,1867,1966,2071,2181,2298,2420],
  H: [10,20,35,50,64,84,93,122,143,174,200,227,259,283,321,365,408,452,493,557,587,640,672,744,779,864,910,958,1016,1080,1150,1226,1307,1394,1431,1530,1591,1658,1774,1852]
};

// Mapping to the library's error correction level constants
export const QR_EC = { L: 1, M: 0, Q: 3, H: 2 }; // library uses: 0=M,1=L,2=H,3=Q

export function minimalVersionForAlnum(len, ecLevel) {
  const arr = CAP_ALNUM[ecLevel];
  if (!arr) return null;
  for (let i = 0; i < arr.length; i++) {
    if (len <= arr[i]) return i + 1; // versions start at 1
  }
  return null; // exceeds version 40
}
