(import (scheme base))

(define (suffixes l)
  (if (null? l)
      (list '())
      (cons l
            (suffixes (cdr l)))))
