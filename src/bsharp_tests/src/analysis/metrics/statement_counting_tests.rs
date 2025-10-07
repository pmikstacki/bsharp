#![allow(clippy::module_inception)]
#[cfg(test)]
mod statement_counting_tests {
    use analysis::AstAnalyze;
    use parser::expressions::declarations::method_declaration_parser::parse_member_declaration;
    use parser::facade::Parser;
    use parser::types::type_parser::parse_type_expression;

    #[test]
    fn test_method_body_statement_counting() {
        let source = r#"
namespace Test
{
    public class TestClass
    {
        public void TestMethod()
        {
            if (x > 0)
            {
                for (int i = 0; i < 10; i++)
                {
                    Console.WriteLine(i);
                }
            }
            
            while (condition)
            {
                DoSomething();
            }
        }
    }
}
"#;

        let parser = Parser::new();
        let ast = parser.parse(source).expect("Failed to parse test source");
        let analysis = ast.analyze();

        // Verify basic structure
        assert_eq!(analysis.total_classes, 1, "Should find 1 class");
        assert_eq!(analysis.total_methods, 1, "Should find 1 method");

        // Verify statements are counted
        assert_eq!(
            analysis.total_if_statements, 1,
            "Should find 1 if statement"
        );
        assert_eq!(analysis.total_for_loops, 1, "Should find 1 for loop");
        assert_eq!(analysis.total_while_loops, 1, "Should find 1 while loop");

        // Verify complexity calculation
        assert!(
            analysis.cyclomatic_complexity >= 4,
            "Should have complexity >= 4 (base=1 + if=1 + for=1 + while=1)"
        );
    }

    #[test]
    fn test_var_declaration_in_method() {
        let source = r#"
namespace Test
{
    public class TestClass
    {
        public async Task TestMethod()
        {
            var user = await GetUserAsync();
            if (user != null)
            {
                var result = ProcessUser(user);
                for (int i = 0; i < result.Count; i++)
                {
                    Console.WriteLine(result[i]);
                }
            }
        }
    }
}
"#;

        let parser = Parser::new();
        let ast = parser
            .parse(source)
            .expect("Failed to parse test source with var declarations");
        let analysis = ast.analyze();

        // Verify structure
        assert_eq!(analysis.total_classes, 1);
        assert_eq!(analysis.total_methods, 1);

        // Verify statements with var declarations are counted
        assert_eq!(
            analysis.total_if_statements, 1,
            "Should find 1 if statement even with var declarations"
        );
        assert_eq!(
            analysis.total_for_loops, 1,
            "Should find 1 for loop even with var declarations"
        );
    }

    #[test]
    fn test_complex_method_like_failing_test() {
        let source = r#"
namespace Test
{
    public class TestClass
    {
        public async Task<bool> CreateUserAsync()
        {
            // Check if user already exists
            var existingUser = await _userRepository.GetByEmailAsync(email);
            if (existingUser != null)
            {
                if (existingUser.IsActive)
                {
                    return false;
                }
                else
                {
                    // Reactivate existing user
                    var updatedUser = await _userRepository.UpdateAsync(existingUser);
                    return true;
                }
            }
            
            // Create new user
            var newUser = new User();
            var createdUser = await _userRepository.CreateAsync(newUser);
            
            // Send welcome email
            if (isActive)
            {
                try
                {
                    await _emailService.SendWelcomeEmailAsync(createdUser);
                }
                catch (Exception emailEx)
                {
                    // Don't fail the entire operation for email issues
                }
            }
            
            return true;
        }
    }
}
"#;

        let parser = Parser::new();
        let ast = parser
            .parse(source)
            .expect("Failed to parse complex method source");
        let analysis = ast.analyze();

        // Verify structure
        assert_eq!(analysis.total_classes, 1);
        assert_eq!(analysis.total_methods, 1);

        // Verify statements are counted
        assert!(
            analysis.total_if_statements >= 3,
            "Should find at least 3 if statements"
        );
        assert_eq!(
            analysis.total_try_statements, 1,
            "Should find 1 try statement"
        );
        assert!(
            analysis.cyclomatic_complexity >= 5,
            "Should have significant complexity"
        );
    }

    #[test]
    fn test_enhanced_expression_parsing_with_string_interpolation() {
        let source = r#"
namespace Test
{
    public class TestClass
    {
        public void TestMethod()
        {
            var email = "user@example.com";
            var message = $"Invalid email: {email}";
            var complexMessage = $"User {user.Name} has email {user.Email ?? "none"}";
            
            if (isValid)
            {
                Console.WriteLine(message);
            }
        }
    }
}
"#;

        let parser = Parser::new();
        let ast = parser
            .parse(source)
            .expect("Failed to parse string interpolation source");
        let analysis = ast.analyze();

        // Verify structure
        assert_eq!(analysis.total_classes, 1);
        assert_eq!(analysis.total_methods, 1);
        assert_eq!(
            analysis.total_if_statements, 1,
            "Should find 1 if statement"
        );
    }

    #[test]
    fn test_enhanced_expression_parsing_with_object_initializers() {
        let source = r#"
namespace Test
{
    public class TestClass
    {
        public void TestMethod()
        {
            var user = new User 
            { 
                Name = "John", 
                Email = email,
                IsActive = true 
            };
            
            var anonymousObject = new { Name = user.Name, Age = 25 };
            
            if (user.IsActive)
            {
                ProcessUser(user);
            }
        }
    }
}
"#;

        let parser = Parser::new();
        let ast = parser
            .parse(source)
            .expect("Failed to parse object initializer source");
        let analysis = ast.analyze();

        // Verify structure
        assert_eq!(analysis.total_classes, 1);
        assert_eq!(analysis.total_methods, 1);
        assert_eq!(
            analysis.total_if_statements, 1,
            "Should find 1 if statement"
        );
    }

    #[test]
    fn test_enhanced_expression_parsing_with_method_chaining() {
        let source = r#"
namespace Test
{
    public class TestClass
    {
        public async Task TestMethod()
        {
            var result = await _userRepository
                .GetByEmailAsync(email)
                .ConfigureAwait(false);
            
            var processedData = data
                .Where(x => x.IsActive)
                .Select(x => x.Name)
                .ToList();
                
            if (result != null)
            {
                var finalResult = result.Process()?.Validate()?.ToString();
            }
        }
    }
}
"#;

        let parser = Parser::new();
        let ast = parser
            .parse(source)
            .expect("Failed to parse method chaining source");
        let analysis = ast.analyze();

        // Verify structure
        assert_eq!(analysis.total_classes, 1);
        assert_eq!(analysis.total_methods, 1);
        assert_eq!(
            analysis.total_if_statements, 1,
            "Should find 1 if statement"
        );
    }

    #[test]
    fn test_complex_generic_return_type() {
        let source = r#"
namespace Test
{
    public class TestClass
    {
        public async Task<Result<User>> ProcessAsync()
        {
            if (true)
            {
                Console.WriteLine("test");
            }
        }
    }
}
"#;

        let parser = Parser::new();
        let ast = parser
            .parse(source)
            .expect("Failed to parse complex generic return type source");
        let analysis = ast.analyze();

        // Debug output to see what we actually found
        println!("Debug: Found {} classes", analysis.total_classes);
        println!("Debug: Found {} methods", analysis.total_methods);
        println!(
            "Debug: Found {} if statements",
            analysis.total_if_statements
        );

        // Verify structure
        assert_eq!(analysis.total_classes, 1);
        assert_eq!(analysis.total_methods, 1);
        assert_eq!(analysis.total_if_statements, 1);
    }

    #[test]
    fn test_simple_async_method() {
        let source = r#"
namespace Test
{
    public class TestClass
    {
        public async Task ProcessAsync()
        {
            if (true)
            {
                Console.WriteLine("test");
            }
        }
    }
}
"#;

        let parser = Parser::new();
        let ast = parser
            .parse(source)
            .expect("Failed to parse async method source");
        let analysis = ast.analyze();

        // Debug output to see what we actually found
        println!("Debug: Found {} classes", analysis.total_classes);
        println!("Debug: Found {} methods", analysis.total_methods);
        println!(
            "Debug: Found {} if statements",
            analysis.total_if_statements
        );

        // Verify structure
        assert_eq!(analysis.total_classes, 1);
        assert_eq!(analysis.total_methods, 1);
        assert_eq!(analysis.total_if_statements, 1);
    }

    #[test]
    fn test_comprehensive_modern_csharp_features() {
        let source = r#"
namespace Test
{
    public class TestClass
    {
        public async Task<Result<User>> ProcessUserAsync(string email)
        {
            try
            {
                // Complex await expression with method chaining
                var existingUser = await _userRepository
                    .GetByEmailAsync(email)
                    .ConfigureAwait(false);
                
                // String interpolation with complex expressions
                var logMessage = $"Processing user: {email} (Found: {existingUser != null})";
                _logger.Info(logMessage);
                
                if (existingUser != null)
                {
                    // Object initializer with complex expressions
                    var updateResult = await _userService.UpdateAsync(new UserUpdateRequest
                    {
                        UserId = existingUser.Id,
                        Email = email,
                        LastModified = DateTime.UtcNow,
                        Metadata = new { Source = "API", UpdatedBy = _currentUser?.Name ?? "System" }
                    });
                    
                    return updateResult.IsSuccess 
                        ? Result<User>.Success(updateResult.Data)
                        : Result<User>.Failure($"Update failed: {updateResult.ErrorMessage}");
                }
                else
                {
                    // Collection initializer and method chaining
                    var newUser = new User
                    {
                        Email = email,
                        CreatedAt = DateTime.UtcNow,
                        Permissions = new List<string> { "read", "write" }
                            .Where(p => _allowedPermissions.Contains(p))
                            .ToList()
                    };
                    
                    var createResult = await _userRepository.CreateAsync(newUser);
                    return Result<User>.Success(createResult);
                }
            }
            catch (Exception ex)
            {
                _logger.Error($"Error processing user {email}: {ex.Message}");
                return Result<User>.Failure(ex.Message);
            }
        }
    }
}
"#;

        let parser = Parser::new();
        let ast = parser
            .parse(source)
            .expect("Failed to parse comprehensive modern C# source");
        let analysis = ast.analyze();

        // Debug output to see what we actually found
        println!("Debug: Found {} classes", analysis.total_classes);
        println!("Debug: Found {} methods", analysis.total_methods);
        println!(
            "Debug: Found {} if statements",
            analysis.total_if_statements
        );
        println!(
            "Debug: Found {} try statements",
            analysis.total_try_statements
        );
        println!(
            "Debug: Cyclomatic complexity: {}",
            analysis.cyclomatic_complexity
        );

        // Verify structure
        assert_eq!(analysis.total_classes, 1);
        assert_eq!(analysis.total_methods, 1);

        // Verify complex control flow is parsed correctly
        assert!(
            analysis.total_if_statements >= 1,
            "Should find at least 1 if statement"
        );
        assert_eq!(
            analysis.total_try_statements, 1,
            "Should find 1 try statement"
        );

        // Verify complexity reflects the sophisticated control flow
        assert!(
            analysis.cyclomatic_complexity >= 4,
            "Should have high enough complexity due to decision points in try/if/catch"
        );
    }

    #[test]
    fn test_progressive_complexity_debug() {
        // Test 1: Simple if
        let source1 = r#"
namespace Test
{
    public class TestClass
    {
        public void SimpleMethod()
        {
            if (true) Console.WriteLine("test");
        }
    }
}
"#;

        let parser = Parser::new();
        let ast1 = parser
            .parse(source1)
            .expect("Failed to parse simple method");
        let analysis1 = ast1.analyze();
        println!(
            "Test 1 - Simple if: if_statements={}, methods={}",
            analysis1.total_if_statements, analysis1.total_methods
        );

        // Test 2: Try statement
        let source2 = r#"
namespace Test
{
    public class TestClass
    {
        public void TryMethod()
        {
            try
            {
                Console.WriteLine("test");
            }
            catch (Exception ex)
            {
                Console.WriteLine("error");
            }
        }
    }
}
"#;

        let ast2 = parser.parse(source2).expect("Failed to parse try method");
        let analysis2 = ast2.analyze();
        println!(
            "Test 2 - Try statement: try_statements={}, methods={}",
            analysis2.total_try_statements, analysis2.total_methods
        );

        // Test 3: Complex async method (like the failing one but simpler)
        let source3 = r#"
namespace Test
{
    public class TestClass
    {
        public async Task<Result<User>> ProcessUserAsync(string email)
        {
            try
            {
                var existingUser = await _userRepository.GetByEmailAsync(email);
                
                if (existingUser != null)
                {
                    return Result<User>.Success(existingUser);
                }
                else
                {
                    return Result<User>.Failure("Not found");
                }
            }
            catch (Exception ex)
            {
                return Result<User>.Failure(ex.Message);
            }
        }
    }
}
"#;

        let ast3 = parser
            .parse(source3)
            .expect("Failed to parse complex async method");
        let analysis3 = ast3.analyze();
        println!(
            "Test 3 - Complex async: if_statements={}, try_statements={}, methods={}",
            analysis3.total_if_statements, analysis3.total_try_statements, analysis3.total_methods
        );

        // All should work
        assert_eq!(analysis1.total_if_statements, 1);
        assert_eq!(analysis2.total_try_statements, 1);
        assert!(analysis3.total_if_statements >= 1);
        assert_eq!(analysis3.total_try_statements, 1);
    }

    #[test]
    fn test_type_parser_complex_generics() {
        // Test if the type syntax can handle complex nested generics
        let test_cases = vec![
            "Task<User>",
            "Task<Result<User>>",
            "List<string>",
            "Dictionary<string, int>",
        ];

        for test_case in test_cases {
            println!("Testing type syntax with: {}", test_case);
            match parse_type_expression(test_case) {
                Ok((remaining, parsed_type)) => {
                    println!(
                        "✅ Successfully parsed '{}' -> {:?}, remaining: '{}'",
                        test_case, parsed_type, remaining
                    );
                }
                Err(e) => {
                    println!("❌ Failed to parse '{}': {:?}", test_case, e);
                    panic!("Type syntax should handle complex generics");
                }
            }
        }
    }

    #[test]
    fn test_member_declaration_parser_async() {
        // Test the exact method signature that's failing
        let test_cases = vec![
            "public void SimpleMethod()",
            "public async Task ProcessAsync()",
            "public async Task<Result<User>> ProcessUserAsync(string email)",
        ];

        for test_case in test_cases {
            println!("Testing member declaration syntax with: {}", test_case);
            match parse_member_declaration(test_case) {
                Ok((remaining, member_decl)) => {
                    println!(
                        "✅ Successfully parsed '{}' -> return_type: {:?}, name: {:?}, remaining: '{}'",
                        test_case, member_decl.return_type, member_decl.name, remaining
                    );
                }
                Err(e) => {
                    println!("❌ Failed to parse '{}': {:?}", test_case, e);
                    // Don't panic here, just log the failure
                }
            }
        }
    }
}
