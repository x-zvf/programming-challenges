##!/usr/bin/env python
import sys

inp = open(sys.argv[1], "r").read()
initl, connl = inp.split("\n\n", 1)
init = {x.split(": ")[0]: x.split(": ")[1] == "1" for x in initl.splitlines()}

conn = []
for l in connl.splitlines():
    if not l.strip():  # Skip empty lines
        continue
    a, op, b, _, r = l.split(" ")
    conn.append((a, b, op, r))


dot = [
    "digraph LogicGates {",
    "    rankdir=LR;",
    "    node [shape=box];",
    "",
    "    # Input values",
]

for wire, value in init.items():
    dot.append(
        f'    {wire} [label="{wire}\\n{1 if value else 0}", style=filled, fillcolor=lightblue];'
    )


gate_counter = 0
for input1, input2, op, output in conn:
    gate_id = f"gate_{gate_counter}"
    dot.append(f'    {gate_id} [label="{op}", shape=circle];')
    dot.append(f'    {input1} -> {gate_id} [label=""];')
    dot.append(f'    {input2} -> {gate_id} [label=""];')
    dot.append(f'    {gate_id} -> {output} [label=""];')
    gate_counter += 1

dot.append("    node [style=filled, fillcolor=lightgreen];")
output_nodes = {conn[i][3] for i in range(len(conn))}
for node in sorted(output_nodes):
    if node.startswith("z"):
        dot.append(f'    {node} [label="{node}"];')

dot.append("}")

output = "\n".join(dot)


print(output)
