(define-library (binary-tree)
  (export empty-tree cons-tree tree? tree-elem tree-left tree-right)
  (import (scheme base))
  (begin
    (define-record-type <tree>
      (cons-tree elem left right)
      tree?
      (elem tree-elem)
      (left tree-left)
      (right tree-right))

    (define empty-tree '())))
