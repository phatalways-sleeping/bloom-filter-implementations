# **Spelling Checker Using Bloom Filter**

## **What is a Bloom Filter?**

A Bloom filter is a data structure that is used to test whether an element is a member of a set. It is a probabilistic data structure that uses multiple hash functions to store the elements of the set.

## **How does a Bloom Filter work?**

A Bloom filter is made of 2 components:

1. An array of m elements: the array is conceptually an array of bits, each of which is initially set to 0

2. k hash functions: these functions are used to hash the elements of the set into the array of bits

We use k bits to store each entry in the Bloom Filter. k, which is the constant picked when creating the data structure, is much smaller than m

When inserting new entry to the data structure:

1. Compute k indices by using k hash functions with the entry as the input

2. Mark the value at k indices in the array as 1 (true)

When searching for an entry:

1. Compute k indices by using k hash functions with the entry as the input

2. If all value at k indices are 1, return true, else false

## **Hashing Functions**

Ideally, we would need **k different independent hash functions** to hash the elements of the set so that no two indices are duplicated for the same value. However, it is not easy to design a large number of independent hash functions.

Good approximations:

1. Using parametric hash function

2. Using a single hash function H but initialize a list L of k random (an unique) values. For each key that is inserted / searched, create k values by appending / adding L[i] to key, then hash them using H

3. Using double or triple hashing

## **Should we use a Bloom Filter for a spelling checker?**

There is no simple yes or no answer to this question. It depends on the use case and the requirements of the application. Althought most of the spelling checker nowadays use tries, here are some pros and cons of using a Bloom Filter for a spelling checker:

### **Pros**

1. **Space-efficient**: A Bloom Filter uses less space compared to a hash table or a trie

2. **Fast**: A Bloom Filter is faster than a hash table or a trie for lookups

### **Cons**

1. **False positives**: A Bloom Filter can return false positives, i.e., it can say that a word is in the dictionary when it is not

2. **Cannot delete entries**: A simple Bloom Filter does not support deletion of entries
