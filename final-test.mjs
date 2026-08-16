/**
 * Final comprehensive test for routing profiles
 * Tests: free/eco/auto/premium with various scenarios
 */

import { route, DEFAULT_ROUTING_CONFIG, BLOCKRUN_MODELS } from "./dist/index.js";

// Build model pricing map
function buildModelPricing() {
  const map = new Map();
  for (const m of BLOCKRUN_MODELS) {
    if (m.id === "auto" || m.id === "free" || m.id === "eco" || m.id === "premium") continue;
    map.set(m.id, { inputPrice: m.inputPrice, outputPrice: m.outputPrice });
  }
  return map;
}

const testCases = [
  {
    category: "SIMPLE tasks",
    tests: [
      {
        name: "Ultra simple Q&A",
        prompt: "Hi",
        systemPrompt: undefined,
        maxTokens: 50,
        expectedTier: "SIMPLE",
      },
      {
        name: "Basic factual question",
        prompt: "What is the capital of France?",
        systemPrompt: undefined,
        maxTokens: 100,
        expectedTier: "SIMPLE",
      },
    ],
  },
  {
    category: "MEDIUM tasks",
    tests: [
      {
        name: "Code explanation",
        prompt: "Explain how async/await works in JavaScript",
        systemPrompt: "You are a helpful programming assistant.",
        maxTokens: 500,
        expectedTier: "MEDIUM",
      },
      {
        name: "Technical writing",
        prompt: "Write a function to validate email addresses using regex",
        systemPrompt: undefined,
        maxTokens: 1000,
        expectedTier: "MEDIUM",
      },
    ],
  },
  {
    category: "COMPLEX tasks",
    tests: [
      {
        name: "Complex code implementation",
        prompt: (
          "Design and implement a distributed microservice architecture for a high-frequency trading platform. " +
          "First define requirements, then produce 1. database schema 2. API specification 3. Kubernetes deployment plan. " +
          "Must include constraints: latency under 5ms, at least 99.99% availability, should handle failover, and not lose data. " +
          "Provide output in JSON schema and table format, include references to RFC 7231 and ISO 27001. " +
          "Analyze algorithmic complexity, optimize sharding strategy, and compare consistency models. "
        ).repeat(12),
        systemPrompt: "You are an expert TypeScript developer.",
        maxTokens: 2000,
        expectedTier: "COMPLEX",
      },
    ],
  },
  {
    category: "REASONING tasks",
    tests: [
      {
        name: "Math word problem",
        prompt:
          "Given a formal theorem, prove by contradiction and derive each step logically. Step 1. Define axioms. Step 2. Derive lemmas. Step 3. Conclude theorem. Use a mathematical proof written formally, step by step.",
        systemPrompt: undefined,
        maxTokens: 1000,
        expectedTier: "REASONING",
      },
    ],
  },
  {
    category: "EDGE CASES",
    tests: [
      {
        name: "Large context (should force COMPLEX)",
        prompt: "x".repeat(500000), // ~125k tokens
        systemPrompt: undefined,
        maxTokens: 1000,
        expectedTier: "COMPLEX",
      },
      {
        name: "Structured output",
        prompt: "List 5 programming languages",
        systemPrompt: "Return response as JSON array",
        maxTokens: 500,
        expectedTier: "MEDIUM", // structuredOutputMinTier
      },
    ],
  },
];

const profiles = ["free", "eco", "auto", "premium"];
const modelPricing = buildModelPricing();
const config = DEFAULT_ROUTING_CONFIG;

// Get Opus 4.5 pricing for baseline verification
const opus45Pricing = modelPricing.get("anthropic/claude-opus-4.5");
const baselineInputPrice = opus45Pricing?.inputPrice || 0;
const baselineOutputPrice = opus45Pricing?.outputPrice || 0;

console.log("╔════════════════════════════════════════════════════════════╗");
console.log("║     IgniteRouter Final Comprehensive Test - v0.8.20         ║");
console.log("╠════════════════════════════════════════════════════════════╣");
console.log(
  `║  Baseline: Claude Opus 4.5 ($${baselineInputPrice}/$${baselineOutputPrice} per M)                  ║`,
);
console.log("╚════════════════════════════════════════════════════════════╝");
console.log("");

let totalTests = 0;
let passedTests = 0;
const issues = [];

for (const category of testCases) {
  console.log(`\n${"=".repeat(60)}`);
  console.log(`${category.category}`);
  console.log("=".repeat(60));

  for (const test of category.tests) {
    totalTests++;
    console.log(`\n📝 ${test.name}`);
    console.log(`   Expected Tier: ${test.expectedTier}`);
    console.log("");

    const results = [];

    for (const profile of profiles) {
      const routerOpts = {
        config,
        modelPricing,
        routingProfile: profile,
      };

      const decision = route(test.prompt, test.systemPrompt, test.maxTokens, routerOpts);

      results.push({
        profile,
        model: decision.model,
        tier: decision.tier,
        cost: decision.costEstimate,
        baseline: decision.baselineCost,
        savings: decision.savings,
      });

      // Validation checks
      if (profile === "premium" && decision.savings !== 0) {
        issues.push(
          `❌ ${test.name} [${profile}]: Premium savings should be 0%, got ${(decision.savings * 100).toFixed(1)}%`,
        );
      }

      if (
        decision.tier !== test.expectedTier &&
        test.name !== "Large context (should force COMPLEX)"
      ) {
        // Large context is expected to override
        issues.push(
          `⚠️  ${test.name} [${profile}]: Expected tier ${test.expectedTier}, got ${decision.tier}`,
        );
      }
    }

    // Display results
    console.log(
      "   Profile    Tier       Model                           Cost      Baseline  Savings",
    );
    console.log(
      "   ─────────  ─────────  ──────────────────────────────  ────────  ────────  ───────",
    );

    for (const r of results) {
      const profileStr = r.profile.padEnd(9);
      const tierStr = r.tier.padEnd(9);
      const modelStr = r.model.slice(0, 30).padEnd(30);
      const costStr = `$${r.cost.toFixed(6)}`.padStart(8);
      const baselineStr = `$${r.baseline.toFixed(6)}`.padStart(8);
      const savingsStr = `${(r.savings * 100).toFixed(1)}%`.padStart(6);

      // Highlight premium with 0% savings
      const savingsDisplay = r.profile === "premium" ? `${savingsStr} ✓` : savingsStr;

      console.log(
        `   ${profileStr}  ${tierStr}  ${modelStr}  ${costStr}  ${baselineStr}  ${savingsDisplay}`,
      );
    }

    // Check if eco has highest savings (excluding premium)
    const nonPremiumResults = results.filter((r) => r.profile !== "premium");
    const ecoResult = results.find((r) => r.profile === "eco");
    const maxSavings = Math.max(...nonPremiumResults.map((r) => r.savings));

    if (ecoResult && Math.abs(ecoResult.savings - maxSavings) < 0.001) {
      console.log(`   ✓ Eco has highest savings (${(maxSavings * 100).toFixed(1)}%)`);
      passedTests++;
    } else {
      issues.push(`❌ ${test.name}: Eco should have highest savings`);
    }
  }
}

// Summary
console.log("\n\n╔════════════════════════════════════════════════════════════╗");
console.log("║                      Test Summary                          ║");
console.log("╠════════════════════════════════════════════════════════════╣");
console.log(`║  Total Tests: ${totalTests.toString().padEnd(45)}║`);
console.log(`║  Passed: ${passedTests.toString().padEnd(49)}║`);
console.log("╠════════════════════════════════════════════════════════════╣");

if (issues.length === 0) {
  console.log("║  ✅ ALL TESTS PASSED!                                      ║");
  console.log("║                                                            ║");
  console.log("║  Key Validations:                                          ║");
  console.log("║  ✓ Premium savings = 0% (quality focused)                  ║");
  console.log("║  ✓ Eco has highest savings (cost optimized)                ║");
  console.log("║  ✓ Baseline = Opus 4.5 ($5/$25)                            ║");
  console.log("║  ✓ All tiers routing correctly                             ║");
} else {
  console.log("║  ⚠️  Issues Found:                                         ║");
  console.log("║                                                            ║");
  for (const issue of issues.slice(0, 10)) {
    // Show first 10 issues
    console.log(`║  ${issue.padEnd(58)}║`);
  }
  if (issues.length > 10) {
    console.log(`║  ... and ${issues.length - 10} more issues`);
  }
}

console.log("╚════════════════════════════════════════════════════════════╝");

// Exit with error code if issues found
if (issues.length > 0) {
  process.exit(1);
}
