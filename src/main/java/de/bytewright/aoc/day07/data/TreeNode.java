package de.bytewright.aoc.day07.data;

public class TreeNode {
    private final long value;
    private Operator operator = null;
    private TreeNode left = null;
    private TreeNode right = null;

    public TreeNode(long value) {
        this.value = value;
    }

    public TreeNode(long value, TreeNode left, TreeNode right, Operator operator) {
        this.value = value;
        this.left = left;
        this.right = right;
        this.operator = operator;
    }

    public long evaluate() {
        if (operator == null) {
            return value;
        }
        return operator.eval(left, right);
    }

    @Override
    public String toString() {
        if (operator == null) {
            return String.valueOf(value);
        }
        return "(" + left + " " + operator + " " + right + ")";
    }

    public long getValue() {
        return this.value;
    }

    public Operator getOperator() {
        return this.operator;
    }

    public TreeNode getLeft() {
        return this.left;
    }

    public TreeNode getRight() {
        return this.right;
    }
}
