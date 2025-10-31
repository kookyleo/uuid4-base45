// QR capacity table for ALPHANUMERIC mode (versions 1..40)
// Actual tested character capacities using qrcode-generator library
// These values account for all overhead: mode indicator, character count, terminator, padding, etc.

export const CAP_ALNUM_CHARS = {
  L: [17,32,53,78,106,134,154,192,230,271,321,367,425,458,520,586,644,718,792,858,929,1003,1091,1171,1273,1367,1465,1528,1628,1732,1840,1952,2068,2188,2303,2431,2563,2699,2809,2953],
  M: [14,26,42,62,84,106,122,152,180,213,251,287,331,362,412,450,504,560,624,666,711,779,857,911,997,1059,1125,1190,1264,1370,1452,1538,1628,1722,1809,1911,1989,2099,2213,2331],
  Q: [11,20,32,46,60,74,86,108,130,151,177,203,241,258,292,322,364,394,442,482,509,565,611,661,715,751,805,868,908,982,1030,1112,1168,1228,1283,1351,1423,1499,1579,1663],
  H: [7,14,24,34,44,58,64,84,98,119,137,155,177,194,220,250,280,310,338,382,403,439,461,511,535,593,625,658,698,742,790,842,898,958,983,1051,1093,1139,1219,1273]
};

// Calculate QR alphanumeric data bits (for display purposes only)
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
