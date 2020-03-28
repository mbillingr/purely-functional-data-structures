(import (scheme base)
        (scheme write)
        (binary-tree))

(define (complete x d)
  (if (< 0 d)
      (let ((subtree (complete x (- d 1))))
        (cons-tree x subtree subtree))
      (cons-tree x empty-tree empty-tree)))

(define (balanced x n)
  (cond ((= 0 n)
         (empty-tree))
        ((= 1 n)
         (cons-tree x empty-tree empty-tree))
        ((= 1 (remainder n 2))
         (let ((subtree (balanced x (/ (- n 1) 2))))
           (cons-tree x subtree subtree)))
        (else
          (create2 x (- (/ n 2) 1)))))

(define (create2 x m)
  (let ((subtree (balanced x m)))))
    
