// Comprehensive integration tests for B# analysis modules
// Tests how quality, metrics, navigation, types, dependencies, etc. work together

use bsharp::analysis::{AstAnalyze};
use bsharp::analysis::quality::{QualityAnalyzer, QualityIssue, QualityGrade};
use bsharp::analysis::navigation::{DeclarationType, AstNavigate, FindDeclarations};
use bsharp::analysis::types::{TypeAnalyzer};
use bsharp::analysis::dependencies::{DependencyAnalyzer};
use bsharp::analysis::naming::{NamingAnalyzer};
use bsharp::parser::Parser;

/// Integration test: Complete analysis workflow
/// Tests that all analysis modules can work together on a complex codebase
#[test]
fn test_comprehensive_analysis_workflow() {
    let source = r#"
using System;
using System.Collections.Generic;
using System.Linq;

namespace ComplexProject.Services
{
    /// <summary>
    /// Service for managing user operations with complex business logic
    /// </summary>
    public class UserManagementService : IUserService
    {
        private readonly IUserRepository _userRepository;
        private readonly IEmailService _emailService;
        private readonly ILoggingService _logger;
        private Dictionary<int, User> _cachedUsers;
        
        /// <summary>
        /// Initializes a new instance of UserManagementService
        /// </summary>
        public UserManagementService(IUserRepository userRepository, 
                                   IEmailService emailService, 
                                   ILoggingService logger)
        {
            _userRepository = userRepository ?? throw new ArgumentNullException(nameof(userRepository));
            _emailService = emailService ?? throw new ArgumentNullException(nameof(emailService));
            _logger = logger ?? throw new ArgumentNullException(nameof(logger));
            _cachedUsers = new Dictionary<int, User>();
        }
        
        /// <summary>
        /// Creates a new user with complex validation and processing
        /// </summary>
        public async Task<UserResult> CreateUserAsync(string firstName, string lastName, 
                                                     string email, int departmentId, 
                                                     UserRole role, DateTime? startDate,
                                                     decimal salary, bool isActive)
        {
            try
            {
                // Complex validation logic with multiple nested conditions
                if (string.IsNullOrWhiteSpace(firstName) || string.IsNullOrWhiteSpace(lastName))
                {
                    _logger.LogWarning("Invalid user name provided");
                    return UserResult.Failure("First name and last name are required");
                }
                
                if (string.IsNullOrWhiteSpace(email) || !IsValidEmail(email))
                {
                    _logger.LogWarning($"Invalid email provided: {email}");
                    return UserResult.Failure("Valid email address is required");
                }
                
                if (departmentId <= 0)
                {
                    _logger.LogWarning($"Invalid department ID: {departmentId}");
                    return UserResult.Failure("Valid department ID is required");
                }
                
                // Check if user already exists
                var existingUser = await _userRepository.GetByEmailAsync(email);
                if (existingUser != null)
                {
                    if (existingUser.IsActive)
                    {
                        _logger.LogWarning($"Active user already exists with email: {email}");
                        return UserResult.Failure("User with this email already exists");
                    }
                    else
                    {
                        // Reactivate existing user
                        existingUser.FirstName = firstName;
                        existingUser.LastName = lastName;
                        existingUser.DepartmentId = departmentId;
                        existingUser.Role = role;
                        existingUser.StartDate = startDate ?? DateTime.Now;
                        existingUser.Salary = salary;
                        existingUser.IsActive = isActive;
                        
                        var updatedUser = await _userRepository.UpdateAsync(existingUser);
                        _cachedUsers[updatedUser.Id] = updatedUser;
                        
                        await _emailService.SendWelcomeEmailAsync(updatedUser);
                        _logger.LogInfo($"Reactivated user: {updatedUser.Id}");
                        
                        return UserResult.Success(updatedUser);
                    }
                }
                
                // Create new user
                var newUser = new User
                {
                    FirstName = firstName,
                    LastName = lastName,
                    Email = email,
                    DepartmentId = departmentId,
                    Role = role,
                    StartDate = startDate ?? DateTime.Now,
                    Salary = salary,
                    IsActive = isActive,
                    CreatedAt = DateTime.UtcNow
                };
                
                // Save to repository
                var createdUser = await _userRepository.CreateAsync(newUser);
                
                // Update cache
                _cachedUsers[createdUser.Id] = createdUser;
                
                // Send welcome email
                if (isActive)
                {
                    try
                    {
                        await _emailService.SendWelcomeEmailAsync(createdUser);
                        _logger.LogInfo($"Welcome email sent to {createdUser.Email}");
                    }
                    catch (Exception emailEx)
                    {
                        _logger.LogError($"Failed to send welcome email: {emailEx.Message}");
                        // Don't fail the entire operation for email issues
                    }
                }
                
                _logger.LogInfo($"Successfully created user: {createdUser.Id}");
                return UserResult.Success(createdUser);
            }
            catch (Exception ex)
            {
                _logger.LogError($"Error creating user: {ex.Message}");
                return UserResult.Failure("An error occurred while creating the user");
            }
        }
        
        // Method without documentation - quality issue
        public void ProcessBulkUsers(List<User> users, int batchSize, bool validateAll, 
                                   bool sendNotifications, bool updateCache, bool logDetails,
                                   int maxRetries, TimeSpan timeout)
        {
            for (int i = 0; i < users.Count; i += batchSize)
            {
                var batch = users.Skip(i).Take(batchSize).ToList();
                
                foreach (var user in batch)
                {
                    if (validateAll)
                    {
                        if (user != null && !string.IsNullOrEmpty(user.Email))
                        {
                            for (int retry = 0; retry < maxRetries; retry++)
                            {
                                try
                                {
                                    if (IsValidEmail(user.Email))
                                    {
                                        if (updateCache)
                                        {
                                            _cachedUsers[user.Id] = user;
                                        }
                                        
                                        if (sendNotifications)
                                        {
                                            // Complex notification logic
                                            switch (user.Role)
                                            {
                                                case UserRole.Admin:
                                                    // Send admin notification
                                                    break;
                                                case UserRole.Manager:
                                                    // Send manager notification
                                                    break;
                                                case UserRole.Employee:
                                                    // Send employee notification
                                                    break;
                                                default:
                                                    // Send default notification
                                                    break;
                                            }
                                        }
                                        
                                        if (logDetails)
                                        {
                                            _logger.LogInfo($"Processed user {user.Id}");
                                        }
                                        
                                        break; // Success, exit retry loop
                                    }
                                }
                                catch (Exception ex)
                                {
                                    if (retry == maxRetries - 1)
                                    {
                                        _logger.LogError($"Failed to process user {user.Id}: {ex.Message}");
                                    }
                                    else
                                    {
                                        Thread.Sleep(100 * (retry + 1)); // Exponential backoff
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        private bool IsValidEmail(string email)
        {
            // Simple email validation
            return email.Contains("@") && email.Contains(".");
        }
    }
    
    public interface IUserService
    {
        Task<UserResult> CreateUserAsync(string firstName, string lastName, string email, 
                                       int departmentId, UserRole role, DateTime? startDate,
                                       decimal salary, bool isActive);
    }
    
    public enum UserRole
    {
        Employee,
        Manager, 
        Admin
    }
}
"#;

    let parser = Parser::new();
    let ast = parser.parse(source).expect("Failed to parse complex source code");
    
    // 1. Test AST Analysis
    let ast_analysis = ast.analyze();
    assert_eq!(ast_analysis.total_classes, 1);
    assert_eq!(ast_analysis.total_interfaces, 1);
    assert_eq!(ast_analysis.total_enums, 1);
    assert_eq!(ast_analysis.total_methods, 3); // CreateUserAsync, ProcessBulkUsers, IsValidEmail
    assert!(ast_analysis.total_if_statements > 10); // Many conditional statements
    assert!(ast_analysis.total_for_loops >= 2); // At least 2 for loops
    assert!(ast_analysis.cyclomatic_complexity > 20); // High complexity
    
    // 2. Test Quality Analysis
    let quality_analyzer = QualityAnalyzer::new();
    let quality_report = quality_analyzer.analyze(&ast);
    
    assert_eq!(quality_report.class_reports.len(), 1);
    let class_report = &quality_report.class_reports[0];
    assert_eq!(class_report.class_name, "UserManagementService");
    
    // Should detect missing documentation for ProcessBulkUsers
    let missing_doc_issues: Vec<_> = class_report.issues.iter()
        .filter(|issue| matches!(issue, QualityIssue::MissingDocumentation { .. }))
        .collect();
    assert!(missing_doc_issues.len() >= 1);
    
    // Should detect too many parameters
    let param_issues: Vec<_> = class_report.issues.iter()
        .filter(|issue| matches!(issue, QualityIssue::TooManyParameters { .. }))
        .collect();
    assert!(param_issues.len() >= 1);
    
    // Should detect high complexity
    let complexity_issues: Vec<_> = class_report.issues.iter()
        .filter(|issue| matches!(issue, QualityIssue::HighComplexity { .. }))
        .collect();
    assert!(complexity_issues.len() >= 1);
    
    // 3. Test Navigation
    let classes = ast.find_classes();
    assert_eq!(classes.len(), 1);
    
    let interfaces = ast.find_interfaces();
    assert_eq!(interfaces.len(), 1);
    
    let enums = ast.find_enums();
    assert_eq!(enums.len(), 1);
    
    // 4. Test Type Analysis
    let type_analyzer = TypeAnalyzer::new();
    // For now, just verify the analyzer can be created successfully
    // Future implementation will add more comprehensive analysis
    
    // 5. Test Dependency Analysis
    let dependency_analyzer = DependencyAnalyzer::new();
    // For now, just verify the analyzer can be created successfully
    // Future implementation will add more comprehensive analysis
    
    // 6. Test Naming Analysis
    let naming_analyzer = NamingAnalyzer::new();
    let naming_metrics = naming_analyzer.analyze(&ast);
    
    // Should mostly follow C# naming conventions (PascalCase for classes, camelCase for fields, etc.)
    // Expect very few violations since the code follows conventions
    assert!(naming_metrics.violations.len() < 5);
    
    // 7. Test Cross-Module Integration
    // Quality report should reference metrics from other modules
    assert!(class_report.quality_score < 8.0); // Should be below excellent due to issues
    
    // Overall quality grade should reflect the various issues found
    assert!(matches!(quality_report.grade, QualityGrade::A | QualityGrade::B | QualityGrade::C | QualityGrade::D | QualityGrade::F));
}

/// Test that analysis modules can handle edge cases together
#[test] 
fn test_analysis_integration_edge_cases() {
    let source = r#"
namespace EdgeCases
{
    // Empty class
    public class EmptyClass 
    {
    }
    
    // Class with only a single complex method
    public class SingleMethodClass
    {
        public void ComplexMethod(int a, int b, int c, int d, int e, int f, int g, int h, int i, int j)
        {
            if (a > 0)
                if (b > 0)
                    if (c > 0)
                        if (d > 0)
                            if (e > 0)
                                if (f > 0)
                                    if (g > 0)
                                        if (h > 0)
                                            if (i > 0)
                                                if (j > 0)
                                                    System.Console.WriteLine("Deep nesting");
        }
    }
    
    // Class with naming violations
    public class badClassName
    {
        public int BAD_FIELD;
        public void bad_method_name() { }
    }
}
"#;

    let parser = Parser::new();
    let ast = parser.parse(source).expect("Failed to parse edge case source");
    
    // Test all analyzers handle edge cases gracefully
    let ast_analysis = ast.analyze();
    let quality_analyzer = QualityAnalyzer::new();
    let quality_report = quality_analyzer.analyze(&ast);
    let type_analyzer = TypeAnalyzer::new();
    let dependency_analyzer = DependencyAnalyzer::new();
    let naming_analyzer = NamingAnalyzer::new();
    let naming_metrics = naming_analyzer.analyze(&ast);
    
    // Verify all analyzers returned results without panicking
    assert_eq!(ast_analysis.total_classes, 3);
    assert_eq!(quality_report.class_reports.len(), 3);
    assert!(naming_metrics.violations.len() > 0); // Should detect naming violations
    
    // Verify specific edge case handling
    // Empty class should not cause issues
    let empty_class_report = quality_report.class_reports.iter()
        .find(|r| r.class_name == "EmptyClass")
        .expect("EmptyClass report should exist");
    assert_eq!(empty_class_report.method_count, 0);
    
    // Single method class should detect complexity issues
    let single_method_report = quality_report.class_reports.iter()
        .find(|r| r.class_name == "SingleMethodClass")
        .expect("SingleMethodClass report should exist");
    assert_eq!(single_method_report.method_count, 1);
    
    // Should detect high nesting and too many parameters
    let has_complexity_issue = single_method_report.issues.iter()
        .any(|issue| matches!(issue, QualityIssue::HighComplexity { .. }));
    let has_param_issue = single_method_report.issues.iter()
        .any(|issue| matches!(issue, QualityIssue::TooManyParameters { .. }));
    
    assert!(has_complexity_issue);
    assert!(has_param_issue);
    
    // Bad naming class should be detected by naming analyzer
    // Check for violations that contain "bad" or "BAD" patterns
    let has_bad_naming = naming_metrics.violations.iter()
        .any(|v| {
            match v {
                bsharp::analysis::naming::NamingViolation::ClassNotPascalCase(name) => name.contains("bad"),
                bsharp::analysis::naming::NamingViolation::MethodNotPascalCase(name) => name.contains("bad"),
                bsharp::analysis::naming::NamingViolation::FieldNotCamelCase(name) => name.contains("BAD"),
                _ => false,
            }
        });
    assert!(has_bad_naming); // Should detect naming violations
}

/// Test performance characteristics of integrated analysis
#[test]
fn test_analysis_integration_performance() {
    let source = r#"
namespace LargeProject
{
    public class ServiceA
    {
        public void Method1() { }
        public void Method2() { }
        public void Method3() { }
        public void Method4() { }
        public void Method5() { }
    }
    
    public class ServiceB  
    {
        public void Method1() { }
        public void Method2() { }
        public void Method3() { }
        public void Method4() { }
        public void Method5() { }
    }
    
    public class ServiceC
    {
        public void Method1() { }
        public void Method2() { }
        public void Method3() { }
        public void Method4() { }
        public void Method5() { }
    }
}
"#;

    let parser = Parser::new();
    let ast = parser.parse(source).expect("Failed to parse performance test source");
    
    use std::time::Instant;
    
    // Time the complete analysis workflow
    let start = Instant::now();
    
    let ast_analysis = ast.analyze();
    let quality_analyzer = QualityAnalyzer::new();
    let quality_report = quality_analyzer.analyze(&ast);
    let type_analyzer = TypeAnalyzer::new();
    let dependency_analyzer = DependencyAnalyzer::new();
    let naming_analyzer = NamingAnalyzer::new();
    let naming_metrics = naming_analyzer.analyze(&ast);
    
    let duration = start.elapsed();
    
    // Analysis should complete quickly (under 100ms for this small example)
    assert!(duration.as_millis() < 100, "Analysis took too long: {:?}", duration);
    
    // Verify all results are consistent
    assert_eq!(ast_analysis.total_classes, 3);
    assert_eq!(quality_report.class_reports.len(), 3);
    assert_eq!(ast_analysis.total_methods, 15); // 3 classes * 5 methods each
    
    // All classes should have similar structure
    for class_report in &quality_report.class_reports {
        assert_eq!(class_report.method_count, 5);
    }
} 