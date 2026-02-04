#import "helpers.typ" as bh
#import "template.typ": conf

// Basic document Properties
#set document(title: [PocketBeagle 2 Coding Workshop])

// Use template
#show: conf.with()

#include "chapter1.typ"

#pagebreak()

#include "chapter2.typ"
