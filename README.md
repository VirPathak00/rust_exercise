# rust_exercise
This code implements rsync's algorithm for computing signatures and computing deltas in rust. The signature is stored in a vec of tuples. Based on the chunk size (block_length in the code) specified, the algorithm will split the file into a number of blocks of size block_length bytes. Then it will compute each block's rolling checksum as specified in the rsync algorithm as well as its strong checksum (ie hash). The signature vec stores all these (checksum, hash tuples).

The deltas are stored in the form Vec<(usize, Vec). The first element i in each tuple denotes a position to start modifying the original file. The second element in the vector stores the elements we will replace the original file with, starting at position i. It will either be a vec containing multiple elements. Or it will be a vec containing a single element j, which means that we must replace bytes [i, i + block_length] with the jth byte block in the original file.

This implementation of rsync makes use of the weak checksum's rolling property as described in the rsync paper. This improves efficiency. However we do not implement its fast checksum search procedure and naively loop through the signature vector instead. This potentially creates a bottleneck when using this implementation on bigger files. I'm pretty sure I can implement the fast checksum search procedure if necessary.

