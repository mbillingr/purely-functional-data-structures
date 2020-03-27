(import (scheme base)
        (scheme write)
        (only (sunny dynwind) call/ep)  ; use call/cc on schemes without call/ep
        (binary-tree)
        (unbalanced-set))

(define numbers (make-set 5 1 3 2 4))

(define (echo-cons-tree x l r)
  (display "consing ")
  (display x)
  (newline)
  (cons-tree x l r))

(define (set-insert set x)
  (call/ep
    (lambda (done)
      (let insert ((subset set)
                   (candidate '()))
        (cond ((set-empty? subset)
               (if (eqv? x candidate)
                   (done set)
                   (echo-cons-tree x empty-tree empty-tree)))
              ((< x (tree-elem subset))
               (echo-cons-tree (tree-elem subset)
                               (insert (tree-left subset) candidate)
                               (tree-right subset)))
              (else
               (echo-cons-tree (tree-elem subset)
                               (tree-left subset)
                               (insert (tree-right subset)
                                       (tree-elem subset)))))))))

(display "Inserting...") (newline)
(set-insert numbers 2)
(display "done") (newline)
