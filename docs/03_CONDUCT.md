# DevBase - Code Conduct & Standards

> **Version:** 1.0  
> **Status:** MANDATORY  
> **Date:** January 19, 2026

---

## ⚠️ IMPORTANT

This document defines the **non-negotiable** rules for writing code in the DevBase project. Every contributor MUST follow these guidelines. Violations will result in rejected PRs.

---

## General Principles

### ✅ DO

| Principle | Description |
|-----------|-------------|
| **Write self-documenting code** | Names should explain intent; comments explain *why*, not *what* |
| **Keep functions small** | Max 50 lines per function; if larger, decompose |
| **Single Responsibility** | One function = one job; one module = one concern |
| **Fail fast, fail loud** | Return errors early; never swallow exceptions silently |
| **Test as you go** | Write tests alongside implementation, not after |
| **Think offline-first** | All features must work without network connectivity |

### ❌ DON'T

| Anti-Pattern | Why |
|--------------|-----|
| **Magic numbers/strings** | Use named constants; `const MAX_REPOS = 1000`, not `1000` |
| **Nested callbacks >2 levels** | Use async/await or proper error propagation |
| **Panic in library code** | Return `Result<T, E>`, let caller decide error handling |
| **Ignore compiler warnings** | Treat warnings as errors; `#![deny(warnings)]` in Rust |
| **Copy-paste code** | Extract to shared function; DRY is law |
| **Commit commented-out code** | Delete it; Git is your history |

---

## Rust Backend Rules

### ✅ DO

```rust
// ✅ Use Result for fallible operations
pub fn scan_directory(path: &Path) -> Result<Vec<Repository>, ScanError> {
    // ...
}

// ✅ Use descriptive error types
#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),
    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),
}

// ✅ Prefer iterators over manual loops
let repo_paths: Vec<_> = entries
    .iter()
    .filter(|e| e.is_git_repo())
    .map(|e| e.path().to_owned())
    .collect();

// ✅ Use builders for complex structs
let config = ScanConfig::builder()
    .max_depth(5)
    .exclude_hidden(true)
    .build()?;
```

### ❌ DON'T

```rust
// ❌ Never use unwrap() in production code
let file = File::open(path).unwrap(); // FORBIDDEN

// ❌ Never use expect() without justification
let data = parse(input).expect(""); // FORBIDDEN - empty message

// ❌ Never block async with std::thread::sleep
std::thread::sleep(Duration::from_secs(1)); // FORBIDDEN in async context

// ❌ Never use unsafe without SAFETY comment
unsafe { /* code */ } // FORBIDDEN without documentation

// ❌ Avoid stringly-typed APIs
fn process(action: &str) { } // BAD - use enum

// ❌ Don't expose internal implementation details
pub struct Scanner {
    pub internal_cache: HashMap<...>, // BAD - should be private
}
```

### Rust-Specific Mandates

| Rule | Enforcement |
|------|-------------|
| Run `cargo clippy` | CI will reject PRs with warnings |
| Run `cargo fmt` | All code must be formatted |
| Use `#[must_use]` | On functions with important return values |
| Document public API | Every `pub fn` needs `///` doc comment |
| No `println!` in library | Use proper logging (`tracing` crate) |

---

## React/TypeScript Frontend Rules

### ✅ DO

```typescript
// ✅ Use TypeScript strict mode - always define types
interface Repository {
  id: string;
  path: string;
  health: HealthStatus;
  lastCommit: Date;
}

// ✅ Use functional components with hooks
const RepoCard: React.FC<RepoCardProps> = ({ repo }) => {
  const [isExpanded, setIsExpanded] = useState(false);
  // ...
};

// ✅ Use custom hooks for reusable logic
function useRepositories() {
  const [repos, setRepos] = useState<Repository[]>([]);
  const [loading, setLoading] = useState(true);
  // ...
  return { repos, loading, refresh };
}

// ✅ Handle all async states
if (loading) return <Skeleton />;
if (error) return <ErrorBoundary error={error} />;
return <RepoList repos={repos} />;

// ✅ Use semantic HTML elements
<article>
  <header>...</header>
  <main>...</main>
  <footer>...</footer>
</article>
```

### ❌ DON'T

```typescript
// ❌ Never use `any` type
function process(data: any) { } // FORBIDDEN

// ❌ Never disable ESLint without justification
// eslint-disable-next-line // FORBIDDEN without comment

// ❌ Never use inline styles for complex styling
<div style={{ marginTop: 10, backgroundColor: 'blue' }}> // BAD

// ❌ Never mutate state directly
repos.push(newRepo); // FORBIDDEN
setRepos([...repos, newRepo]); // CORRECT

// ❌ Never mix business logic in components
const RepoCard = () => {
  // ❌ BAD - API call inside render
  fetch('/api/repos').then(...);
};

// ❌ Avoid prop drilling >3 levels
<A><B><C><D prop={value} /></C></B></A> // Use context instead
```

### TypeScript/React Mandates

| Rule | Enforcement |
|------|-------------|
| Enable `strict: true` | tsconfig.json requirement |
| Run `eslint` | CI will reject PRs with warnings |
| Run `prettier` | All code must be formatted |
| No unused variables | `noUnusedLocals: true` |
| Test with React Testing Library | Not Enzyme |
| Accessibility via `eslint-plugin-jsx-a11y` | Required |

---

## File & Folder Structure

### ✅ DO

```
src-tauri/
├── src/
│   ├── commands/      # Tauri IPC commands
│   ├── scanner/       # Repository scanning
│   ├── git/           # Git operations
│   ├── db/            # Database layer
│   ├── search/        # Ripgrep integration
│   └── lib.rs         # Library root
│
src/
├── components/        # Reusable UI components
│   ├── common/        # Buttons, inputs, etc.
│   ├── dashboard/     # Dashboard-specific
│   └── viewer/        # Viewer-specific
├── hooks/             # Custom React hooks
├── stores/            # State management
├── services/          # API/Tauri communication
├── types/             # TypeScript types
└── utils/             # Helper functions
```

### ❌ DON'T

- Put business logic in `components/`
- Create files >400 lines (decompose!)
- Use generic names like `utils.ts`, `helpers.ts`, `misc.rs`
- Mix test files with source files (use `__tests__/` folders)

---

## Git Workflow Rules

### ✅ DO

| Practice | Description |
|----------|-------------|
| **Atomic commits** | One logical change per commit |
| **Conventional commits** | `feat:`, `fix:`, `docs:`, `refactor:`, `test:` |
| **Descriptive branches** | `feat/global-search`, `fix/scanner-symlinks` |
| **Squash before merge** | Clean history in main branch |
| **Sign commits** | Use GPG signing |

### ❌ DON'T

| Anti-Pattern | Example |
|--------------|---------|
| Vague commit messages | `"fixes"`, `"updates"`, `"wip"` |
| Force push to main | **NEVER** |
| Large PRs (>500 LOC) | Split into smaller PRs |
| Commit secrets | API keys, passwords, tokens |
| Commit generated files | `node_modules/`, `target/`, `.DS_Store` |

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Example:**
```
feat(scanner): add recursive .git directory detection

- Implements depth-first search for .git folders
- Adds configurable max depth limit
- Excludes node_modules by default

Closes #42
```

---

## Testing Standards

### ✅ DO

| Practice | Description |
|----------|-------------|
| **Test behavior, not implementation** | Test what function does, not how |
| **Use descriptive test names** | `test_scanner_ignores_symlinks_by_default` |
| **One assertion per test** | Focus on single behavior |
| **Use fixtures/factories** | Don't repeat test data setup |
| **Test edge cases** | Empty inputs, large inputs, Unicode |

### ❌ DON'T

| Anti-Pattern | Why |
|--------------|-----|
| Test private functions | Test through public API only |
| Mock everything | Integration tests have value |
| Skip flaky tests | Fix them or delete them |
| Write tests after code is "done" | TDD or concurrent development |
| Ignore test coverage drops | Maintain ≥80% backend, ≥70% frontend |

### Test File Naming

```
# Rust
src/scanner/mod.rs        # Source
src/scanner/tests.rs      # Unit tests

# TypeScript
src/hooks/useRepos.ts          # Source
src/hooks/__tests__/useRepos.test.ts  # Tests
```

---

## Performance Rules

### ✅ DO

| Practice | Description |
|----------|-------------|
| **Lazy load** | Don't load 1000 repos at startup |
| **Virtualize lists** | Use react-window for long lists |
| **Cache expensive operations** | Memoize Git operations |
| **Use indexes** | SQLite queries must use indexes |
| **Profile before optimizing** | Measure, don't guess |

### ❌ DON'T

| Anti-Pattern | Why |
|--------------|-----|
| Premature optimization | Readability > micro-optimization |
| Blocking main thread | Use Web Workers/Rust threads |
| Memory leaks | Clean up subscriptions, watchers |
| N+1 queries | Batch database operations |
| Load entire files for preview | Stream/paginate large content |

---

## Security Rules

### ✅ DO

| Practice | Description |
|----------|-------------|
| **Validate all inputs** | Paths, regex patterns, user data |
| **Sanitize file paths** | Prevent path traversal attacks |
| **Use allowlists** | Prefer allowlist over denylist |
| **Minimal permissions** | Request only necessary FS access |
| **Audit dependencies** | Run `cargo audit`, `npm audit` |

### ❌ DON'T

| Anti-Pattern | Why |
|--------------|-----|
| Execute arbitrary shell commands | Use libgit2, not `git` CLI |
| Store sensitive data unencrypted | Use OS keychain if needed |
| Trust user-provided paths blindly | Canonicalize and validate |
| Ignore CVE warnings | Update vulnerable deps immediately |
| Log sensitive information | No passwords, tokens in logs |

---

## Documentation Standards

### ✅ DO

| Practice | Description |
|----------|-------------|
| **Document "why"** | Code shows "what," docs show "why" |
| **Update docs with code** | Stale docs are worse than none |
| **Use examples** | Show, don't just tell |
| **README per module** | Complex modules need explanation |
| **API docs for public interfaces** | Every `pub fn` needs `///` |

### ❌ DON'T

| Anti-Pattern | Why |
|--------------|-----|
| Document obvious code | `// increment i` before `i += 1` |
| Use TODOs as permanent comments | Create issues instead |
| Leave WIP markers | `// FIXME` must be resolved or ticketed |
| Write docs once and forget | Review docs during PR reviews |

---

## Code Review Checklist

Every PR must pass this checklist:

- [ ] Code follows all rules in this document
- [ ] All tests pass (`cargo test`, `npm test`)
- [ ] Linters pass (`cargo clippy`, `eslint`)
- [ ] New code has corresponding tests
- [ ] Documentation is updated
- [ ] No security vulnerabilities introduced
- [ ] Commit messages follow convention
- [ ] PR is reasonably sized (<500 LOC)
- [ ] Breaking changes are documented
- [ ] No `TODO`/`FIXME` added without issue link

---

## Enforcement

| Mechanism | Description |
|-----------|-------------|
| **CI Pipeline** | Automated lint, test, security checks |
| **Pre-commit hooks** | Local format/lint before commit |
| **PR Templates** | Enforce checklist completion |
| **Code Review** | At least 1 approval required |
| **Periodic Audits** | Monthly code health reviews |

---

> **Remember:** These rules exist to maintain a healthy, maintainable codebase. When in doubt, ask. When rules conflict with common sense, discuss and update the rules.
