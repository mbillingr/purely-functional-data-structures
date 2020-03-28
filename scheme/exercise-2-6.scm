(import (scheme base)
        (binary-tree))

(define empty-map empty-tree)

(define (make-map . assoc)
  (let loop ((assoc assoc)
             (map empty-map))
    (if (null? assoc)
        map
        (loop (cdr items) (map-bind map (caar items) (cdar items))))))

(define (map-empty? map)
  (eq? map empty-map))

(define (map-lookup map key)
  (cond ((map-empty? map) #f)
        ((< key (car (tree-elem map)))
         (map-lookup (tree-left map) key))
        ((eqv? key (car (tree-elem map)))
         (tree-elem map))
        (else (map-lookup (tree-right map) key))))

(define (map-bind map key val)
  (cond ((map-empty? map)
         (cons-tree (cons key val) empty-tree empty-tree))
        ((< key (car (tree-elem map)))
         (cons-tree (tree-elem map)
                    (map-bind (tree-left map) key val)
                    (tree-right map)))
        ((eqv? key (car (tree-elem map)))
         (cons-tree (cons key val)
                    (tree-left map)
                    (tree-right map)))
        (else (cons-tree (tree-elem map)
                         (tree-left map)
                         (map-bind (tree-right map) key val)))))

(define-syntax map-bind!
  (syntax-rules ()
    ((map-bind! map key val)
     (set! map (map-bind map key val)))))
