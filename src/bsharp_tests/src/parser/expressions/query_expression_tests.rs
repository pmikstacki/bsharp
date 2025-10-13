// Tests for parsing query expressions

use parser::expressions::query_expression_parser::parse_query_expression;
use syntax::expressions::expression::Expression;
use syntax::expressions::query_expression::*;

fn parse_query_test(code: &str) -> Result<Expression, String> {
    match parse_query_expression(code) {
        Ok((remaining, expr)) => {
            if remaining.trim().is_empty() {
                Ok(expr)
            } else {
                Err(format!("Unparsed input: {}", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_simple_query_expression() {
    let code = "from x in collection select x";
    let result = parse_query_test(code);
    assert!(
        result.is_ok(),
        "Failed to parse simple query expression: {:?}",
        result
    );

    if let Ok(Expression::Query(query)) = result {
        // Check from clause
        assert_eq!(query.from.identifier.name, "x");
        assert!(query.from.type_annotation.is_none());

        // Check that there are no intermediate body clauses
        assert_eq!(query.body.len(), 0);

        // Check select clause
        if let QuerySelectOrGroup::Select(select_expr) = &query.select_or_group {
            if let Expression::Variable(var) = select_expr {
                assert_eq!(var.name, "x");
            } else {
                panic!("Expected variable in select clause");
            }
        } else {
            panic!("Expected select clause");
        }

        // Check no continuation
        assert!(query.continuation.is_none());
    } else {
        panic!("Expected query expression");
    }
}

#[test]
fn test_parse_query_with_type_annotation() {
    let code = "from int x in numbers select x";
    let result = parse_query_test(code);
    assert!(
        result.is_ok(),
        "Failed to parse query with type annotation: {:?}",
        result
    );

    if let Ok(Expression::Query(query)) = result {
        assert!(query.from.type_annotation.is_some());
        assert_eq!(query.from.type_annotation.as_ref().unwrap().name, "int");
        assert_eq!(query.from.identifier.name, "x");
    } else {
        panic!("Expected query expression");
    }
}

#[test]
fn test_parse_query_with_where_clause() {
    let code = "from x in collection where x > 5 select x";
    let result = parse_query_test(code);
    assert!(
        result.is_ok(),
        "Failed to parse query with where clause: {:?}",
        result
    );

    if let Ok(Expression::Query(query)) = result {
        assert_eq!(query.body.len(), 1);

        if let QueryClause::Where(where_clause) = &query.body[0] {
            // The where condition should be a binary expression (x > 5)
            if let Expression::Binary { left, .. } = &where_clause.condition {
                if let Expression::Variable(var) = left.as_ref() {
                    assert_eq!(var.name, "x");
                } else {
                    panic!("Expected variable in where condition left side");
                }
            } else {
                panic!("Expected binary expression in where condition");
            }
        } else {
            panic!("Expected where clause in body");
        }
    } else {
        panic!("Expected query expression");
    }
}

#[test]
fn test_parse_query_with_let_clause() {
    let code = "from x in collection let doubled = x * 2 select doubled";
    let result = parse_query_test(code);
    assert!(
        result.is_ok(),
        "Failed to parse query with let clause: {:?}",
        result
    );

    if let Ok(Expression::Query(query)) = result {
        assert_eq!(query.body.len(), 1);

        if let QueryClause::Let(let_clause) = &query.body[0] {
            assert_eq!(let_clause.identifier.name, "doubled");

            // The let expression should be a binary expression (x * 2)
            if let Expression::Binary { left, .. } = &let_clause.expression {
                if let Expression::Variable(var) = left.as_ref() {
                    assert_eq!(var.name, "x");
                } else {
                    panic!("Expected variable in let expression left side");
                }
            } else {
                panic!("Expected binary expression in let clause");
            }
        } else {
            panic!("Expected let clause in body");
        }
    } else {
        panic!("Expected query expression");
    }
}

#[test]
fn test_parse_query_with_orderby_clause() {
    let code = "from x in collection orderby x select x";
    let result = parse_query_test(code);
    assert!(
        result.is_ok(),
        "Failed to parse query with orderby clause: {:?}",
        result
    );

    if let Ok(Expression::Query(query)) = result {
        assert_eq!(query.body.len(), 1);

        if let QueryClause::OrderBy(orderby_clause) = &query.body[0] {
            assert_eq!(orderby_clause.orderings.len(), 1);

            let ordering = &orderby_clause.orderings[0];
            if let Expression::Variable(var) = &ordering.expression {
                assert_eq!(var.name, "x");
            } else {
                panic!("Expected variable in orderby expression");
            }

            // Default should be ascending (None means ascending)
            assert!(
                ordering.direction.is_none()
                    || matches!(ordering.direction, Some(OrderingDirection::Ascending))
            );
        } else {
            panic!("Expected orderby clause in body");
        }
    } else {
        panic!("Expected query expression");
    }
}

#[test]
fn test_parse_query_with_orderby_descending() {
    let code = "from x in collection orderby x descending select x";
    let result = parse_query_test(code);
    assert!(
        result.is_ok(),
        "Failed to parse query with orderby descending: {:?}",
        result
    );

    if let Ok(Expression::Query(query)) = result {
        assert_eq!(query.body.len(), 1);

        if let QueryClause::OrderBy(orderby_clause) = &query.body[0] {
            let ordering = &orderby_clause.orderings[0];
            assert!(matches!(
                ordering.direction,
                Some(OrderingDirection::Descending)
            ));
        } else {
            panic!("Expected orderby clause in body");
        }
    } else {
        panic!("Expected query expression");
    }
}

#[test]
fn test_parse_query_with_group_clause() {
    let code = "from x in collection group x by x.Category";
    let result = parse_query_test(code);
    assert!(
        result.is_ok(),
        "Failed to parse query with group clause: {:?}",
        result
    );

    if let Ok(Expression::Query(query)) = result {
        if let QuerySelectOrGroup::Group { element, by } = &query.select_or_group {
            if let Expression::Variable(var) = element {
                assert_eq!(var.name, "x");
            } else {
                panic!("Expected variable in group element");
            }

            if let Expression::MemberAccess(member_access) = by {
                assert_eq!(member_access.member.name, "Category");
            } else {
                panic!("Expected member access in group by clause");
            }
        } else {
            panic!("Expected group clause");
        }
    } else {
        panic!("Expected query expression");
    }
}

#[test]
fn test_parse_complex_query_expression() {
    let code = "from x in collection where x.Age > 18 let adult = true orderby x.Name select new { x.Name, adult }";
    let result = parse_query_test(code);
    assert!(
        result.is_ok(),
        "Failed to parse complex query expression: {:?}",
        result
    );

    if let Ok(Expression::Query(query)) = result {
        // Should have where, let, and orderby clauses
        assert_eq!(query.body.len(), 3);

        // Check where clause
        assert!(matches!(query.body[0], QueryClause::Where(_)));

        // Check let clause
        assert!(matches!(query.body[1], QueryClause::Let(_)));

        // Check orderby clause
        assert!(matches!(query.body[2], QueryClause::OrderBy(_)));

        // Check select clause (should be anonymous object creation)
        if let QuerySelectOrGroup::Select(select_expr) = &query.select_or_group {
            assert!(matches!(select_expr, Expression::AnonymousObject(_)));
        } else {
            panic!("Expected select clause with anonymous object");
        }
    } else {
        panic!("Expected query expression");
    }
}

#[test]
fn test_parse_query_with_join_clause() {
    let code = "from customer in customers join order in orders on customer.Id equals order.CustomerId select customer";
    let result = parse_query_test(code);
    assert!(
        result.is_ok(),
        "Failed to parse query with join clause: {:?}",
        result
    );

    if let Ok(Expression::Query(query)) = result {
        assert_eq!(query.body.len(), 1);

        if let QueryClause::Join(join_clause) = &query.body[0] {
            assert_eq!(join_clause.identifier.name, "order");
            assert!(join_clause.into_identifier.is_none());
        } else {
            panic!("Expected join clause in body");
        }
    } else {
        panic!("Expected query expression");
    }
}

#[test]
fn test_parse_query_with_multiple_from_clauses() {
    let code = "from customer in customers from order in customer.Orders select order";
    let result = parse_query_test(code);
    assert!(
        result.is_ok(),
        "Failed to parse query with multiple from clauses: {:?}",
        result
    );

    if let Ok(Expression::Query(query)) = result {
        assert_eq!(query.body.len(), 1);

        if let QueryClause::From(from_clause) = &query.body[0] {
            assert_eq!(from_clause.identifier.name, "order");
        } else {
            panic!("Expected additional from clause in body");
        }
    } else {
        panic!("Expected query expression");
    }
}
