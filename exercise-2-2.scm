(import (scheme base)
        (binary-tree)
        (unbalanced-set))

; better number of worst case comparisons (depth + 1 instead of 2 * depth) but
; no early stopping...

(define (set-member? set x)
  (let search ((subset set)
               (candidate '()))
    (cond ((set-empty? subset) #f
           (eqv? candidate x))
          ((< x (tree-elem subset))
           (search (tree-left subset) candidate))
          (else
           (search (tree-right subset) (tree-elem subset))))))
