#!/usr/bin/env python3
# Generates index.html for gh-pages with two tabs: Conformance and Coverage.
# Usage: generate-pages-html.py <junit.xml> <conformance.json> <run_url> <hex> <passed> <total> <pct> <cov_pct>

import sys
import json
import xml.etree.ElementTree as ET
from collections import defaultdict
from datetime import datetime, timezone

junit_path, conf_path, run_url, hex_color, passed, total, pct, cov_pct = sys.argv[1:9]

cov_float = float(cov_pct)
if   cov_float >= 80.0: cov_hex = "#4c1"
elif cov_float >= 60.0: cov_hex = "#3c3"
elif cov_float >= 40.0: cov_hex = "#db1"
else:                   cov_hex = "#e05"

# ── Unit tests per category ───────────────────────────────────────────────────
tree = ET.parse(junit_path)
root = tree.getroot()
unit_counts = defaultdict(int)
for suite in root.findall('testsuite'):
    if suite.get('name') == 'truecalc-core':
        for tc in suite.findall('testcase'):
            parts = tc.get('name', '').split('::')
            cat = 'core'
            if 'functions' in parts:
                idx = parts.index('functions')
                if idx + 1 < len(parts) and parts[idx + 1] != 'tests':
                    cat = parts[idx + 1]
            unit_counts[cat] += 1

# ── Proptest suites per category ──────────────────────────────────────────────
SKIP = {'error_propagation', 'conformance'}
CASES = 500
prop_fns = defaultdict(int)
for suite in root.findall('testsuite'):
    sname = suite.get('name', '')
    if sname.startswith('truecalc-core::property_'):
        cat = sname[len('truecalc-core::property_'):]
        if cat not in SKIP:
            prop_fns[cat] += int(suite.get('tests', 0))

# ── Conformance data ──────────────────────────────────────────────────────────
conf = json.load(open(conf_path))
cats_conf = conf.get('by_category', {})

all_cats = sorted(
    (set(unit_counts.keys()) | set(prop_fns.keys()) | set(cats_conf.keys())) - {'core'}
)

# ── Build conformance table rows ──────────────────────────────────────────────
rows_html = []
tot_unit = tot_prop = tot_grand = 0

for cat in all_cats:
    unit = unit_counts.get(cat, 0)
    fn_c = prop_fns.get(cat, 0)
    cd = cats_conf.get(cat)
    cp = cd['passed'] if cd else 0
    ct = cd['total'] if cd else 0

    if cd:
        cls = 'pass' if cp == ct else 'fail'
        mark = '✓' if cp == ct else '⚠'
        conf_cell = f'<span class="{cls}">{cp:,}/{ct:,} {mark}</span>'
    else:
        conf_cell = '—'

    if fn_c:
        prop_cases = fn_c * CASES
        prop_cell = f'{prop_cases:,} ({fn_c}×{CASES:,})'
    else:
        prop_cases = 0
        prop_cell = '—'

    row_total = unit + ct + prop_cases
    display = cat.replace('_', ' ').title()
    rows_html.append(
        f'<tr><td>{display}</td><td>{unit:,}</td>'
        f'<td>{conf_cell}</td><td>{prop_cell}</td><td>{row_total:,}</td></tr>'
    )
    tot_unit += unit
    tot_prop += prop_cases
    tot_grand += unit + ct + prop_cases

core_unit = unit_counts.get('core', 0)
tot_unit += core_unit
tot_grand += core_unit
tot_prop_fns = sum(prop_fns.values())

updated = datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M UTC')

print(f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Test Transparency — truecalc</title>
  <style>
    *, *::before, *::after {{ box-sizing: border-box; }}
    body {{ font-family: system-ui, sans-serif; max-width: 960px; margin: 40px auto; padding: 0 20px; color: #24292f; }}
    h1 {{ font-size: 1.5rem; margin-bottom: 4px; }}
    .subtitle {{ color: #57606a; font-size: 0.9rem; margin-bottom: 24px; }}

    /* ── Tabs ── */
    .tab-bar {{ display: flex; gap: 0; border-bottom: 2px solid #d0d7de; margin-bottom: 24px; }}
    .tab-btn {{
      padding: 10px 20px; font-size: 0.95rem; font-weight: 600;
      background: none; border: none; cursor: pointer; color: #57606a;
      border-bottom: 3px solid transparent; margin-bottom: -2px;
      transition: color 0.15s;
    }}
    .tab-btn:hover {{ color: #24292f; }}
    .tab-btn.active {{ color: #0969da; border-bottom-color: #0969da; }}
    .tab-pane {{ display: none; }}
    .tab-pane.active {{ display: block; }}

    /* ── Badges ── */
    .badges {{ margin: 0 0 20px; display: flex; gap: 8px; flex-wrap: wrap; align-items: center; }}
    .badge {{ display: inline-block; color: #fff; padding: 5px 14px; border-radius: 4px; font-size: 1.05rem; font-weight: bold; }}

    /* ── Explainer ── */
    .explainer {{ background: #f6f8fa; border: 1px solid #d0d7de; border-radius: 6px; padding: 12px 16px; margin: 0 0 20px; font-size: 0.9rem; line-height: 1.6; }}
    .explainer strong {{ color: #24292f; }}

    /* ── Table ── */
    table {{ border-collapse: collapse; width: 100%; margin-top: 4px; }}
    th, td {{ border: 1px solid #d0d7de; padding: 8px 12px; text-align: left; }}
    th {{ background: #f6f8fa; font-size: 0.875rem; }}
    tfoot td {{ font-weight: bold; background: #f6f8fa; }}
    .pass {{ color: #1a7f37; }}
    .fail {{ color: #cf222e; }}

    /* ── Coverage iframe ── */
    .coverage-wrap {{
      border: 1px solid #d0d7de; border-radius: 6px; overflow: hidden;
      margin-top: 4px;
    }}
    .coverage-wrap iframe {{
      width: 100%; height: 78vh; border: none; display: block;
    }}

    /* ── Footer ── */
    footer {{ margin-top: 32px; padding-top: 16px; border-top: 1px solid #d0d7de; font-size: 0.85rem; color: #57606a; }}
  </style>
</head>
<body>
  <h1>truecalc — Test Transparency</h1>
  <p class="subtitle">
    <a href="{run_url}">CI run</a> &nbsp;·&nbsp; Updated: {updated}
  </p>

  <div class="tab-bar">
    <button class="tab-btn active" data-tab="conformance">Google Sheets Conformance</button>
    <button class="tab-btn"        data-tab="coverage">Code Coverage</button>
  </div>

  <!-- ── Conformance tab ── -->
  <div id="tab-conformance" class="tab-pane active">
    <div class="badges">
      <span class="badge" style="background:{hex_color}">{passed}/{total} · {pct}%</span>
    </div>

    <div class="explainer">
      <strong>What is Google Sheets conformance?</strong><br>
      truecalc evaluates spreadsheet formulas. To verify correctness, every supported formula is run against
      a <em>Google Sheets oracle</em> — real Google Sheets spreadsheets that produce the expected output.
      On every commit to <code>main</code>, truecalc re-runs all {total} conformance cases and compares
      results. A ✓ means truecalc matches Google Sheets exactly; ⚠ means a known, intentional deviation
      (e.g. a locale difference or an unsupported edge case).<br><br>
      <strong>Property tests</strong> go further: for each formula category, randomly generated inputs
      are checked against mathematical invariants (e.g. <code>ABS(x) ≥ 0</code> for all x,
      <code>SQRT(x)² ≈ x</code> for x &gt; 0). Each property runs {CASES:,} random cases per commit.
    </div>

    <table>
      <thead>
        <tr>
          <th>Category</th>
          <th>Unit Tests</th>
          <th>Google Sheets Conformance</th>
          <th>Property Cases</th>
          <th>Total</th>
        </tr>
      </thead>
      <tbody>
        {''.join(rows_html)}
      </tbody>
      <tfoot>
        <tr>
          <td>Total</td>
          <td>{tot_unit:,}</td>
          <td>{conf['passed']:,}/{conf['total']:,}</td>
          <td>{tot_prop:,} ({tot_prop_fns}×{CASES:,})</td>
          <td>~{tot_grand:,}</td>
        </tr>
      </tfoot>
    </table>
  </div>

  <!-- ── Coverage tab ── -->
  <div id="tab-coverage" class="tab-pane">
    <div class="badges">
      <span class="badge" style="background:{cov_hex}">coverage · {cov_pct}%</span>
    </div>

    <div class="explainer">
      <strong>What is code coverage?</strong><br>
      Line coverage is measured by <code>cargo-llvm-cov</code> on the <code>truecalc-core</code> crate.
      It tracks which source lines are executed across all unit tests, integration tests, conformance
      tests, and property tests combined. A higher percentage means more of the implementation
      has been exercised by the test suite.
    </div>

    <div class="coverage-wrap">
      <iframe src="coverage/" title="Full coverage report"></iframe>
    </div>
  </div>

  <footer>
    Oracle: Google Sheets &nbsp;·&nbsp;
    ✓ = 100% passing &nbsp;·&nbsp; ⚠ = known deviation
  </footer>

  <script>
    (function () {{
      var btns  = document.querySelectorAll('.tab-btn');
      var panes = document.querySelectorAll('.tab-pane');

      function activate(id) {{
        btns.forEach(function(b)  {{ b.classList.toggle('active', b.dataset.tab === id); }});
        panes.forEach(function(p) {{ p.classList.toggle('active', p.id === 'tab-' + id); }});
        history.replaceState(null, '', '#' + id);
      }}

      btns.forEach(function(b) {{
        b.addEventListener('click', function() {{ activate(b.dataset.tab); }});
      }});

      // Honour URL hash on load
      var hash = location.hash.replace('#', '');
      if (hash === 'conformance' || hash === 'coverage') {{ activate(hash); }}
    }})();
  </script>
</body>
</html>""")
