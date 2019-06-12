Day 56: LZW
<br>
Lempel–Ziv–Welch (LZW) https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Welch is a universal lossless data compression algorithm created by Abraham Lempel, Jacob Ziv, and Terry Welch. It was published by Welch in 1984 as an improved implementation of the LZ78 algorithm published by Lempel and Ziv in 1978. The algorithm is simple to implement and has the potential for very high throughput in hardware implementations.[1] It is the algorithm of the widely used Unix file compression utility compress and is used in the GIF image format.

<br>
LZW iteratively builds a (code, word) dictionary as it reads input data. At the beginning, the whole alphabet is inserted and hence needs not to be stored to output.
<br>
Input is then matched to the longest known word in dictionary and its code is sent to output. The word extended by the next input character is stored as a new entry.
<br>
This technique is fast as it needs no extra search inside large blocks. And unlike my implementation with dict, it just needs an array with pointers.
<br>
However, major flaw of the algorithm are the dictionary codes. To store the code, we have to periodically enlarge number of bits from 9 up to 12 or 16. At this point, dictionary is usually erased and algorithm starts over.
<br>
That empowers its weakness — the dictionary is built very slowly. Once it contains enough information to compress efficiently, it is erased. But allowing larger size is tricky due to long codes and decrease in performance.
<br>
run:
```
fn main() {
    let mut lzw = LZW::new();
    let text = "BABAABAAA";
    let encoded = lzw.encode(text.as_bytes());
    let decoded = lzw.decode(&encoded);
    assert_eq!(String::from_utf8(decoded).unwrap(), text);
}
```