# Deque and RandomizedQueue

```
javac-algs4 Permutation.java

java-algs4 Permutation 3 < distinct.txt
java-algs4 Permutation 9 < duplicates.txt
```

* As it mentions `must support each deque operation (including construction) in constant worst-case time`
so we need to use a double linked list to implement `deque`;
* As it mentions `must support each randomized queue operation (besides creating an iterator) in constant amortized time`
so we need to use an array implementation