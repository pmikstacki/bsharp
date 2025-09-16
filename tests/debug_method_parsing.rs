use bsharp::parser::declarations::method_declaration_parser::parse_member_declaration;
use bsharp::parser::statement_parser::debug_test_individual_parsers;

#[test]
fn test_complex_async_method_parsing() {
    let input = r#"public async Task<Result<User>> ProcessUserAsync(string email)
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
        }"#;

    println!("Testing complex async method parsing...");
    match parse_member_declaration(input) {
        Ok((remaining, member)) => {
            println!("✅ Method parsed successfully!");
            println!("Return type: {:?}", member.return_type);
            println!("Method name: {}", member.name.name);
            println!("Has body: {}", member.body.is_some());
            println!("Remaining input: {:?}", remaining);
        }
        Err(e) => {
            println!("❌ Method parsing failed: {:?}", e);
        }
    }
}

#[test]
fn test_simple_method_parsing() {
    let input = r#"public void SimpleMethod()
        {
            if (true)
            {
                Console.WriteLine("Hello");
            }
        }"#;

    println!("Testing simple method parsing...");
    match parse_member_declaration(input) {
        Ok((remaining, member)) => {
            println!("✅ Simple method parsed successfully!");
            println!("Return type: {:?}", member.return_type);
            println!("Method name: {}", member.name.name);
            println!("Has body: {}", member.body.is_some());
            println!("Remaining input: {:?}", remaining);
        }
        Err(e) => {
            println!("❌ Simple method parsing failed: {:?}", e);
        }
    }
}

#[test]
fn test_method_with_simple_try() {
    let input = r#"public void TestTryMethod()
        {
            try
            {
                Console.WriteLine("test");
            }
            catch (Exception ex)
            {
                Console.WriteLine("error");
            }
        }"#;

    println!("Testing method with simple try statement...");
    match parse_member_declaration(input) {
        Ok((remaining, member)) => {
            println!("✅ Try method parsed successfully!");
            println!("Return type: {:?}", member.return_type);
            println!("Method name: {}", member.name.name);
            println!("Has body: {}", member.body.is_some());
            println!("Remaining input: {:?}", remaining);
        }
        Err(e) => {
            println!("❌ Try method parsing failed: {:?}", e);
        }
    }
}

#[test]
fn test_method_with_progressive_complexity() {
    // Test 1: Just var declaration
    let input1 = r#"public void TestMethod()
        {
            try
            {
                var existingUser = GetUser();
            }
            catch (Exception ex)
            {
                Console.WriteLine("error");
            }
        }"#;

    println!("Testing method with simple var declaration...");
    match parse_member_declaration(input1) {
        Ok((_remaining, member)) => {
            println!("✅ Simple var method parsed successfully! Has body: {}", member.body.is_some());
        }
        Err(e) => {
            println!("❌ Simple var method parsing failed: {:?}", e);
        }
    }

    // Test 2: Await expression
    let input2 = r#"public async Task TestMethod()
        {
            try
            {
                var existingUser = await _userRepository.GetByEmailAsync(email);
            }
            catch (Exception ex)
            {
                Console.WriteLine("error");
            }
        }"#;

    println!("Testing method with await expression...");
    match parse_member_declaration(input2) {
        Ok((_remaining, member)) => {
            println!("✅ Await method parsed successfully! Has body: {}", member.body.is_some());
        }
        Err(e) => {
            println!("❌ Await method parsing failed: {:?}", e);
        }
    }

    // Test 3: Method chaining
    let input3 = r#"public async Task TestMethod()
        {
            try
            {
                var existingUser = await _userRepository
                    .GetByEmailAsync(email)
                    .ConfigureAwait(false);
            }
            catch (Exception ex)
            {
                Console.WriteLine("error");
            }
        }"#;

    println!("Testing method with method chaining...");
    match parse_member_declaration(input3) {
        Ok((_remaining, member)) => {
            println!("✅ Method chaining parsed successfully! Has body: {}", member.body.is_some());
        }
        Err(e) => {
            println!("❌ Method chaining parsing failed: {:?}", e);
        }
    }
}

#[test]
fn test_method_with_string_interpolation() {
    let input = r#"public async Task TestMethod()
        {
            try
            {
                var existingUser = await _userRepository.GetByEmailAsync(email);
                var logMessage = $"Processing user: {email} (Found: {existingUser != null})";
                _logger.Info(logMessage);
            }
            catch (Exception ex)
            {
                Console.WriteLine("error");
            }
        }"#;

    println!("Testing method with string interpolation...");
    match parse_member_declaration(input) {
        Ok((_remaining, member)) => {
            println!("✅ String interpolation method parsed successfully! Has body: {}", member.body.is_some());
        }
        Err(e) => {
            println!("❌ String interpolation method parsing failed: {:?}", e);
        }
    }
}

#[test]
fn test_incremental_complexity() {
    // Test 1: Just the first few statements
    let input1 = r#"public async Task TestMethod()
        {
            try
            {
                var existingUser = await _userRepository
                    .GetByEmailAsync(email)
                    .ConfigureAwait(false);
                var logMessage = $"Processing user: {email} (Found: {existingUser != null})";
            }
            catch (Exception ex)
            {
                Console.WriteLine("error");
            }
        }"#;

    println!("Testing method with first two statements...");
    match parse_member_declaration(input1) {
        Ok((_remaining, member)) => {
            println!("✅ First two statements parsed successfully! Has body: {}", member.body.is_some());
        }
        Err(e) => {
            println!("❌ First two statements parsing failed: {:?}", e);
        }
    }

    // Test 2: Add method call
    let input2 = r#"public async Task TestMethod()
        {
            try
            {
                var existingUser = await _userRepository
                    .GetByEmailAsync(email)
                    .ConfigureAwait(false);
                var logMessage = $"Processing user: {email} (Found: {existingUser != null})";
                _logger.Info(logMessage);
            }
            catch (Exception ex)
            {
                Console.WriteLine("error");
            }
        }"#;

    println!("Testing method with method call...");
    match parse_member_declaration(input2) {
        Ok((_remaining, member)) => {
            println!("✅ Method call parsed successfully! Has body: {}", member.body.is_some());
        }
        Err(e) => {
            println!("❌ Method call parsing failed: {:?}", e);
        }
    }

    // Test 3: Add if statement
    let input3 = r#"public async Task TestMethod()
        {
            try
            {
                var existingUser = await _userRepository
                    .GetByEmailAsync(email)
                    .ConfigureAwait(false);
                var logMessage = $"Processing user: {email} (Found: {existingUser != null})";
                _logger.Info(logMessage);
                
                if (existingUser != null)
                {
                    Console.WriteLine("Found user");
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine("error");
            }
        }"#;

    println!("Testing method with if statement...");
    match parse_member_declaration(input3) {
        Ok((_remaining, member)) => {
            println!("✅ If statement parsed successfully! Has body: {}", member.body.is_some());
        }
        Err(e) => {
            println!("❌ If statement parsing failed: {:?}", e);
        }
    }
}

#[test]
fn test_debug_first_statement() {
    // Test just the first var statement from the complex method
    let first_statement = r#"var existingUser = await _userRepository
                    .GetByEmailAsync(email)
                    .ConfigureAwait(false);"#;

    println!("Testing the first statement from the complex method...");
    println!("{}", debug_test_individual_parsers(first_statement));
} 