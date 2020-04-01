(define-library (heap)
  (export empty-heap heap-empty? make-heap
          heap
          heap-insert heap-find-min heap-del-min heap-merge heap-rank)
  (import (scheme base)
          (binary-tree))
  (begin
    (define empty-heap '())

    (define (heap-empty? set)
      (null? set))

    (define (heap . items)
      (if (null? items)
          empty-heap
          (heap-insert (apply heap (cdr items))
                       (car items))))

    (define-record-type <heap>
      (make-heap rank elem left right)
      heap?
      (rank heap-r)
      (elem heap-elem)
      (left heap-left)
      (right heap-right))

    (define (heap-insert h x)
      (heap-merge (make-heap 1 x empty-heap empty-heap)
                  h))

    (define heap-find-min heap-elem)

    (define (heap-del-min h)
      (heap-merge (heap-left h)
                  (heap-right h)))

    (define (heap-merge h1 h2)
      (cond ((heap-empty? h1) h2)
            ((heap-empty? h2) h1)
            ((< (heap-elem h1)
                (heap-elem h2))
             (make-t (heap-elem h1)
                     (heap-left h1)
                     (heap-merge (heap-right h1)
                                 h2)))
            (else
              (make-t (heap-elem h2)
                      (heap-left h2)
                      (heap-merge h1
                                  (heap-right h2))))))

    (define (heap-rank h)
      (if (heap-empty? h)
          0
          (heap-r h)))

    (define (make-t x a b)
      (if (< (heap-rank b) (heap-rank a))
          (make-heap (+ 1 (heap-rank b))
                     x a b)
          (make-heap (+ 1 (heap-rank a))
                     x b a)))))
