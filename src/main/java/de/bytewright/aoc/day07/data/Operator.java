package de.bytewright.aoc.day07.data;

import java.util.Objects;

public enum Operator {
    ADD("+"),
    MULTIPLY("*"), CONCAT("||");

    final String symbol;

    Operator(String symbol) {
        this.symbol = symbol;
    }

    @Override
    public String toString() {
        return symbol;
    }

    public long eval(TreeNode leftTree, TreeNode rightTree) {
        Objects.requireNonNull(leftTree);
        Objects.requireNonNull(rightTree);
        return switch (this) {
            case ADD -> leftTree.getValue() + rightTree.getValue();
            case MULTIPLY -> leftTree.getValue() * rightTree.getValue();
            case CONCAT -> Long.parseLong(leftTree.getValue() + "" + rightTree.getValue());
        };
    }

    public long eval(long arg1, long arg2) {
        return switch (this) {
            case ADD -> arg1 + arg2;
            case MULTIPLY -> arg1 * arg2;
            case CONCAT -> Long.parseLong(arg1 + "" + arg2);
        };
    }

    public String symbol() {
        return symbol;
    }
}
