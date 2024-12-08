package de.bytewright.aoc.day07;

import de.bytewright.aoc.day07.data.Operator;
import lombok.Getter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.LinkedList;
import java.util.List;
import java.util.concurrent.atomic.AtomicLong;

@Getter
public class TreeNode2 {
    private static final Logger LOGGER = LoggerFactory.getLogger(TreeNode2.class);
    private final List<TreeNode2> leafNodes;
    private final AtomicLong nodeCounter;
    private final long arg;
    private final long evalToThisNode;
    private final Operator operator;
    private final TreeNode2 parent;
    private List<TreeNode2> childNodes;

    public TreeNode2(TreeNode2 parent, long currentArg, long eval, Operator currentOp) {
        this.parent = parent;
        this.arg = currentArg;
        this.evalToThisNode = eval;
        this.operator = currentOp;
        if (parent == null) {
            nodeCounter = new AtomicLong(1L); // this
        } else {
            nodeCounter = null; // save memory
        }
        leafNodes = new LinkedList<>();
    }

    public long getNodeCount() {
        return nodeCounter.get();
    }

    public void addLeafNode(TreeNode2 leafNodes) {
        this.leafNodes.add(leafNodes);
    }

    public List<TreeNode2> getPath() {
        List<TreeNode2> path = new LinkedList<>();
        path.add(this);
        do {
            path.add(0, path.get(0)
                            .getParent());
        } while (path.get(0)
                     .getParent() != null);
        return path;
    }

    public void addChildren(List<TreeNode2> childNodes) {
        this.childNodes = childNodes;
    }

    public String asStringEq() {
        if (operator == null) {
            return arg + " ";
        }
        return operator.symbol() + " " + arg + " ";
    }

    public void incNodeCount() {
        nodeCounter.incrementAndGet();
    }
}
