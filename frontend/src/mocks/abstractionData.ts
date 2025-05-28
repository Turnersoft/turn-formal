/**
 * Mock data for group theory abstraction levels
 * This simulates the response from the Rust backend
 */
export const mockAbstractionData = {
  "l1_group_schema_full": {
    "id": "l1_schema-groupbasic-section",
    "title": {
      "segments": [{"Text": "Group Theory"}],
      "alignment": null
    },
    "content": [
      {
        "Paragraph": {
          "segments": [{"Text": "A group is an algebraic structure consisting of a set with a binary operation that satisfies four fundamental properties: closure, associativity, identity, and invertibility."}],
          "alignment": null
        }
      },
      {
        "SubSection": {
          "id": "l1_schema-definition-section",
          "title": {"segments": [{"Text": "Definition"}], "alignment": null},
          "content": [
            {
              "Paragraph": {
                "segments": [{"Text": "A group (G, *) consists of a set G and a binary operation * such that:"}],
                "alignment": null
              }
            },
            {
              "Paragraph": {
                "segments": [{"Text": "1. Closure: For all a, b ∈ G, a * b ∈ G"}],
                "alignment": null
              }
            },
            {
              "Paragraph": {
                "segments": [{"Text": "2. Associativity: For all a, b, c ∈ G, (a * b) * c = a * (b * c)"}],
                "alignment": null
              }
            },
            {
              "Paragraph": {
                "segments": [{"Text": "3. Identity: There exists an element e ∈ G such that for all a ∈ G, e * a = a * e = a"}],
                "alignment": null
              }
            },
            {
              "Paragraph": {
                "segments": [{"Text": "4. Inverse: For each a ∈ G, there exists an element a⁻¹ ∈ G such that a * a⁻¹ = a⁻¹ * a = e"}],
                "alignment": null
              }
            }
          ],
          "metadata": null,
          "display_options": null
        }
      },
      {
        "SubSection": {
          "id": "l1_schema-properties-section",
          "title": {"segments": [{"Text": "Properties"}], "alignment": null},
          "content": [
            {
              "Paragraph": {
                "segments": [{"Text": "Groups can have various properties:"}],
                "alignment": null
              }
            },
            {
              "Paragraph": {
                "segments": [{"Text": "- Abelian/Commutative: If a * b = b * a for all a, b ∈ G"}],
                "alignment": null
              }
            },
            {
              "Paragraph": {
                "segments": [{"Text": "- Finite/Infinite: Whether the group has a finite or infinite number of elements"}],
                "alignment": null
              }
            }
          ],
          "metadata": null,
          "display_options": null
        }
      }
    ],
    "metadata": {"type": "GroupBasicDefinition", "abstraction_level": "1"},
    "display_options": null
  },
  "l1_group_schema_tooltip": {
    "id": "l1_schema_tooltip",
    "title": {"segments": [{"Text": "Group Theory"}], "alignment": null},
    "content": [
      {
        "Paragraph": {
          "segments": [{"Text": "Group Theory - Abstract algebraic structure with a binary operation satisfying closure, associativity, identity, and inverses"}],
          "alignment": null
        }
      }
    ],
    "metadata": {"display_mode": "tooltip", "abstraction_level": "1"},
    "display_options": null
  },
  "l1_group_schema_reference": {
    "id": "l1_schema_reference",
    "title": null,
    "content": [
      {
        "Paragraph": {
          "segments": [
            {
              "Link": {
                "content": [{"Text": "Group Theory"}],
                "target": {"DefinitionId": {"term_id": "l1_schema-l1-doc", "theory_context": "GroupTheory"}},
                "tooltip": "View definition of Group Theory"
              }
            }
          ],
          "alignment": null
        }
      }
    ],
    "metadata": {"display_mode": "reference", "abstraction_level": "1"},
    "display_options": null
  },
  "l2_group_type_0_full": {
    "id": "l2_type_0-group-section",
    "title": {"segments": [{"Text": "Abelian Group"}], "alignment": null},
    "content": [
      {
        "StructuredMath": {
          "Definition": {
            "term_display": [{"Text": "Abelian Group"}],
            "formal_term": {"id": "l2_type_0-formalTerm", "content": {"Text": "(G, *)"}},
            "label": "Definition (Abelian Group)",
            "body": [
              {
                "Paragraph": {
                  "segments": [{"Text": "This is a group where the binary operation is commutative."}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "For all elements a, b ∈ G, we have a * b = b * a."}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "Properties:"}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "- Abelian (Commutative)"}],
                  "alignment": null
                }
              }
            ],
            "abstraction_meta": {"level": 2},
            "selectable_properties": [
              {
                "name": "Commutativity",
                "current_variant": "Abelian",
                "all_variants": ["Abelian", "NonAbelian"],
                "description": "Is commutative?"
              }
            ]
          }
        }
      }
    ],
    "metadata": {"type": "GroupDefinition"},
    "display_options": null
  },
  "l2_group_type_0_tooltip": {
    "id": "l2_type_0_tooltip",
    "title": {"segments": [{"Text": "Abelian Group"}], "alignment": null},
    "content": [
      {
        "Paragraph": {
          "segments": [{"Text": "A group where elements commute (a*b = b*a for all elements)"}],
          "alignment": null
        }
      }
    ],
    "metadata": {"display_mode": "tooltip", "abstraction_level": "2"},
    "display_options": null
  },
  "l2_group_type_0_reference": {
    "id": "l2_type_0_reference",
    "title": null,
    "content": [
      {
        "Paragraph": {
          "segments": [
            {
              "Link": {
                "content": [{"Text": "Abelian Group"}],
                "target": {"DefinitionId": {"term_id": "l2_type_0-group-section", "theory_context": "GroupTheory"}},
                "tooltip": "View definition of Abelian Group"
              }
            }
          ],
          "alignment": null
        }
      }
    ],
    "metadata": {"display_mode": "reference", "abstraction_level": "2"},
    "display_options": null
  },
  "l2_group_type_2_full": {
    "id": "l2_type_2-group-section",
    "title": {"segments": [{"Text": "Symmetric Group S_4"}], "alignment": null},
    "content": [
      {
        "StructuredMath": {
          "Definition": {
            "term_display": [{"Text": "Symmetric Group S_4"}],
            "formal_term": {"id": "l2_type_2-formalTerm", "content": {"Text": "S_4"}},
            "label": "Definition (Symmetric Group S_4)",
            "body": [
              {
                "Paragraph": {
                  "segments": [{"Text": "Degree: 4"}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "Order: 24"}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "The group of all permutations on 4 elements."}],
                  "alignment": null
                }
              }
            ],
            "abstraction_meta": {"level": 2},
            "selectable_properties": null
          }
        }
      }
    ],
    "metadata": {"type": "GroupDefinition"},
    "display_options": null
  },
  "l3_group_constructor_0_full": {
    "id": "l3_constructor_0-group-section",
    "title": {"segments": [{"Text": "Cyclic Group C_5"}], "alignment": null},
    "content": [
      {
        "StructuredMath": {
          "Definition": {
            "term_display": [{"Text": "Cyclic Group C_5"}],
            "formal_term": {"id": "l3_constructor_0-formalTerm", "content": {"Text": "C_5"}},
            "label": "Definition (Cyclic Group C_5)",
            "body": [
              {
                "Paragraph": {
                  "segments": [{"Text": "Generator: g"}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "Order: 5"}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "A cyclic group is generated by a single element, with all other elements being powers of the generator."}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "C_5 = {e, g, g², g³, g⁴} where g⁵ = e"}],
                  "alignment": null
                }
              }
            ],
            "abstraction_meta": {"level": 3},
            "selectable_properties": null
          }
        }
      }
    ],
    "metadata": {"type": "GroupDefinition"},
    "display_options": null
  },
  "l3_group_constructor_0_tooltip": {
    "id": "l3_constructor_0_tooltip",
    "title": {"segments": [{"Text": "Cyclic Group Constructor"}], "alignment": null},
    "content": [
      {
        "Paragraph": {
          "segments": [{"Text": "Creates a cyclic group C_n with order n, generated by a single element"}],
          "alignment": null
        }
      }
    ],
    "metadata": {"display_mode": "tooltip", "abstraction_level": "3"},
    "display_options": null
  },
  "l4_group_instance_0_full": {
    "id": "l4_instance_0-group-section",
    "title": {"segments": [{"Text": "Z/6Z (Integers mod 6)"}], "alignment": null},
    "content": [
      {
        "StructuredMath": {
          "Definition": {
            "term_display": [{"Text": "Z/6Z (Integers mod 6)"}],
            "formal_term": {"id": "l4_instance_0-formalTerm", "content": {"Text": "Z/6Z"}},
            "label": "Concrete Group (Z/6Z)",
            "body": [
              {
                "Paragraph": {
                  "segments": [{"Text": "Concrete group on set {0, 1, 2, 3, 4, 5} with explicit structure:"}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "Operation: Addition modulo 6"}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "Identity element: 0"}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "Elements: {0, 1, 2, 3, 4, 5}"}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "Properties with concrete values:"}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "- Finite(6)"}],
                  "alignment": null
                }
              },
              {
                "Paragraph": {
                  "segments": [{"Text": "- Abelian(Abelian)"}],
                  "alignment": null
                }
              }
            ],
            "abstraction_meta": {"level": 4},
            "selectable_properties": [
              {
                "name": "Order",
                "current_variant": "Finite(6)",
                "all_variants": ["Finite(usize)", "Infinite", "LocallyFinite"],
                "description": "Order of the group."
              },
              {
                "name": "Commutativity",
                "current_variant": "Abelian(Abelian)",
                "all_variants": ["Abelian", "NonAbelian"],
                "description": "Is commutative?"
              }
            ]
          }
        }
      }
    ],
    "metadata": {"type": "GroupDefinition"},
    "display_options": null
  },
  "l4_group_instance_0_tooltip": {
    "id": "l4_instance_0_tooltip",
    "title": {"segments": [{"Text": "Z/6Z"}], "alignment": null},
    "content": [
      {
        "Paragraph": {
          "segments": [{"Text": "Integers modulo 6 under addition: {0, 1, 2, 3, 4, 5}"}],
          "alignment": null
        }
      }
    ],
    "metadata": {"display_mode": "tooltip", "abstraction_level": "4"},
    "display_options": null
  },
  "l4_group_instance_0_reference": {
    "id": "l4_instance_0_reference",
    "title": null,
    "content": [
      {
        "Paragraph": {
          "segments": [
            {
              "Link": {
                "content": [{"Text": "Z/6Z"}],
                "target": {"DefinitionId": {"term_id": "l4_instance_0-group-section", "theory_context": "GroupTheory"}},
                "tooltip": "View definition of Z/6Z"
              }
            }
          ],
          "alignment": null
        }
      }
    ],
    "metadata": {"display_mode": "reference", "abstraction_level": "4"},
    "display_options": null
  }
}; 