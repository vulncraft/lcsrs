#!/usr/bin/env bash
# Seed lcsrs with the NeetCode 150.
# Source: https://github.com/neetcode-gh/leetcode/blob/main/.problemSiteData.json
# Generated: 2026-05-24
set -euo pipefail

LCSRS="${LCSRS:-lcsrs}"

# --- Arrays & Hashing ---
"$LCSRS" add 'Contains Duplicate' https://leetcode.com/problems/contains-duplicate --difficulty easy --tag array --tag hash-table
"$LCSRS" add 'Valid Anagram' https://leetcode.com/problems/valid-anagram --difficulty easy --tag array --tag hash-table
"$LCSRS" add 'Two Sum' https://leetcode.com/problems/two-sum --difficulty easy --tag array --tag hash-table
"$LCSRS" add 'Group Anagrams' https://leetcode.com/problems/group-anagrams --difficulty medium --tag array --tag hash-table
"$LCSRS" add 'Top K Frequent Elements' https://leetcode.com/problems/top-k-frequent-elements --difficulty medium --tag array --tag hash-table
"$LCSRS" add 'Product of Array Except Self' https://leetcode.com/problems/product-of-array-except-self --difficulty medium --tag array --tag hash-table
"$LCSRS" add 'Valid Sudoku' https://leetcode.com/problems/valid-sudoku --difficulty medium --tag array --tag hash-table
"$LCSRS" add 'Encode and Decode Strings' https://leetcode.com/problems/encode-and-decode-strings --difficulty medium --tag array --tag hash-table
"$LCSRS" add 'Longest Consecutive Sequence' https://leetcode.com/problems/longest-consecutive-sequence --difficulty medium --tag array --tag hash-table
# --- Two Pointers ---
"$LCSRS" add 'Valid Palindrome' https://leetcode.com/problems/valid-palindrome --difficulty easy --tag two-pointers
"$LCSRS" add 'Two Sum II Input Array Is Sorted' https://leetcode.com/problems/two-sum-ii-input-array-is-sorted --difficulty medium --tag two-pointers
"$LCSRS" add 3Sum https://leetcode.com/problems/3sum --difficulty medium --tag two-pointers
"$LCSRS" add 'Container With Most Water' https://leetcode.com/problems/container-with-most-water --difficulty medium --tag two-pointers
"$LCSRS" add 'Trapping Rain Water' https://leetcode.com/problems/trapping-rain-water --difficulty hard --tag two-pointers
# --- Sliding Window ---
"$LCSRS" add 'Best Time to Buy And Sell Stock' https://leetcode.com/problems/best-time-to-buy-and-sell-stock --difficulty easy --tag sliding-window
"$LCSRS" add 'Longest Substring Without Repeating Characters' https://leetcode.com/problems/longest-substring-without-repeating-characters --difficulty medium --tag sliding-window
"$LCSRS" add 'Longest Repeating Character Replacement' https://leetcode.com/problems/longest-repeating-character-replacement --difficulty medium --tag sliding-window
"$LCSRS" add 'Permutation In String' https://leetcode.com/problems/permutation-in-string --difficulty medium --tag sliding-window
"$LCSRS" add 'Minimum Window Substring' https://leetcode.com/problems/minimum-window-substring --difficulty hard --tag sliding-window
"$LCSRS" add 'Sliding Window Maximum' https://leetcode.com/problems/sliding-window-maximum --difficulty hard --tag sliding-window
# --- Stack ---
"$LCSRS" add 'Valid Parentheses' https://leetcode.com/problems/valid-parentheses --difficulty easy --tag stack
"$LCSRS" add 'Min Stack' https://leetcode.com/problems/min-stack --difficulty medium --tag stack
"$LCSRS" add 'Evaluate Reverse Polish Notation' https://leetcode.com/problems/evaluate-reverse-polish-notation --difficulty medium --tag stack
"$LCSRS" add 'Generate Parentheses' https://leetcode.com/problems/generate-parentheses --difficulty medium --tag stack
"$LCSRS" add 'Daily Temperatures' https://leetcode.com/problems/daily-temperatures --difficulty medium --tag stack
"$LCSRS" add 'Car Fleet' https://leetcode.com/problems/car-fleet --difficulty medium --tag stack
"$LCSRS" add 'Largest Rectangle In Histogram' https://leetcode.com/problems/largest-rectangle-in-histogram --difficulty hard --tag stack
# --- Binary Search ---
"$LCSRS" add 'Binary Search' https://leetcode.com/problems/binary-search --difficulty easy --tag binary-search
"$LCSRS" add 'Search a 2D Matrix' https://leetcode.com/problems/search-a-2d-matrix --difficulty medium --tag binary-search
"$LCSRS" add 'Koko Eating Bananas' https://leetcode.com/problems/koko-eating-bananas --difficulty medium --tag binary-search
"$LCSRS" add 'Find Minimum In Rotated Sorted Array' https://leetcode.com/problems/find-minimum-in-rotated-sorted-array --difficulty medium --tag binary-search
"$LCSRS" add 'Search In Rotated Sorted Array' https://leetcode.com/problems/search-in-rotated-sorted-array --difficulty medium --tag binary-search
"$LCSRS" add 'Time Based Key Value Store' https://leetcode.com/problems/time-based-key-value-store --difficulty medium --tag binary-search
"$LCSRS" add 'Median of Two Sorted Arrays' https://leetcode.com/problems/median-of-two-sorted-arrays --difficulty hard --tag binary-search
# --- Linked List ---
"$LCSRS" add 'Reverse Linked List' https://leetcode.com/problems/reverse-linked-list --difficulty easy --tag linked-list
"$LCSRS" add 'Merge Two Sorted Lists' https://leetcode.com/problems/merge-two-sorted-lists --difficulty easy --tag linked-list
"$LCSRS" add 'Reorder List' https://leetcode.com/problems/reorder-list --difficulty medium --tag linked-list
"$LCSRS" add 'Remove Nth Node From End of List' https://leetcode.com/problems/remove-nth-node-from-end-of-list --difficulty medium --tag linked-list
"$LCSRS" add 'Copy List With Random Pointer' https://leetcode.com/problems/copy-list-with-random-pointer --difficulty medium --tag linked-list
"$LCSRS" add 'Add Two Numbers' https://leetcode.com/problems/add-two-numbers --difficulty medium --tag linked-list
"$LCSRS" add 'Linked List Cycle' https://leetcode.com/problems/linked-list-cycle --difficulty easy --tag linked-list
"$LCSRS" add 'Find The Duplicate Number' https://leetcode.com/problems/find-the-duplicate-number --difficulty medium --tag linked-list
"$LCSRS" add 'LRU Cache' https://leetcode.com/problems/lru-cache --difficulty medium --tag linked-list
"$LCSRS" add 'Merge K Sorted Lists' https://leetcode.com/problems/merge-k-sorted-lists --difficulty hard --tag linked-list
"$LCSRS" add 'Reverse Nodes In K Group' https://leetcode.com/problems/reverse-nodes-in-k-group --difficulty hard --tag linked-list
# --- Trees ---
"$LCSRS" add 'Invert Binary Tree' https://leetcode.com/problems/invert-binary-tree --difficulty easy --tag tree
"$LCSRS" add 'Maximum Depth of Binary Tree' https://leetcode.com/problems/maximum-depth-of-binary-tree --difficulty easy --tag tree
"$LCSRS" add 'Diameter of Binary Tree' https://leetcode.com/problems/diameter-of-binary-tree --difficulty easy --tag tree
"$LCSRS" add 'Balanced Binary Tree' https://leetcode.com/problems/balanced-binary-tree --difficulty easy --tag tree
"$LCSRS" add 'Same Tree' https://leetcode.com/problems/same-tree --difficulty easy --tag tree
"$LCSRS" add 'Subtree of Another Tree' https://leetcode.com/problems/subtree-of-another-tree --difficulty easy --tag tree
"$LCSRS" add 'Lowest Common Ancestor of a Binary Search Tree' https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree --difficulty medium --tag tree
"$LCSRS" add 'Binary Tree Level Order Traversal' https://leetcode.com/problems/binary-tree-level-order-traversal --difficulty medium --tag tree
"$LCSRS" add 'Binary Tree Right Side View' https://leetcode.com/problems/binary-tree-right-side-view --difficulty medium --tag tree
"$LCSRS" add 'Count Good Nodes In Binary Tree' https://leetcode.com/problems/count-good-nodes-in-binary-tree --difficulty medium --tag tree
"$LCSRS" add 'Validate Binary Search Tree' https://leetcode.com/problems/validate-binary-search-tree --difficulty medium --tag tree
"$LCSRS" add 'Kth Smallest Element In a Bst' https://leetcode.com/problems/kth-smallest-element-in-a-bst --difficulty medium --tag tree
"$LCSRS" add 'Construct Binary Tree From Preorder And Inorder Traversal' https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal --difficulty medium --tag tree
"$LCSRS" add 'Binary Tree Maximum Path Sum' https://leetcode.com/problems/binary-tree-maximum-path-sum --difficulty hard --tag tree
"$LCSRS" add 'Serialize And Deserialize Binary Tree' https://leetcode.com/problems/serialize-and-deserialize-binary-tree --difficulty hard --tag tree
# --- Tries ---
"$LCSRS" add 'Implement Trie Prefix Tree' https://leetcode.com/problems/implement-trie-prefix-tree --difficulty medium --tag trie
"$LCSRS" add 'Design Add And Search Words Data Structure' https://leetcode.com/problems/design-add-and-search-words-data-structure --difficulty medium --tag trie
"$LCSRS" add 'Word Search II' https://leetcode.com/problems/word-search-ii --difficulty hard --tag trie
# --- Heap / Priority Queue ---
"$LCSRS" add 'Kth Largest Element In a Stream' https://leetcode.com/problems/kth-largest-element-in-a-stream --difficulty easy --tag heap --tag priority-queue
"$LCSRS" add 'Last Stone Weight' https://leetcode.com/problems/last-stone-weight --difficulty easy --tag heap --tag priority-queue
"$LCSRS" add 'K Closest Points to Origin' https://leetcode.com/problems/k-closest-points-to-origin --difficulty medium --tag heap --tag priority-queue
"$LCSRS" add 'Kth Largest Element In An Array' https://leetcode.com/problems/kth-largest-element-in-an-array --difficulty medium --tag heap --tag priority-queue
"$LCSRS" add 'Task Scheduler' https://leetcode.com/problems/task-scheduler --difficulty medium --tag heap --tag priority-queue
"$LCSRS" add 'Design Twitter' https://leetcode.com/problems/design-twitter --difficulty medium --tag heap --tag priority-queue
"$LCSRS" add 'Find Median From Data Stream' https://leetcode.com/problems/find-median-from-data-stream --difficulty hard --tag heap --tag priority-queue
# --- Backtracking ---
"$LCSRS" add Subsets https://leetcode.com/problems/subsets --difficulty medium --tag backtracking
"$LCSRS" add 'Combination Sum' https://leetcode.com/problems/combination-sum --difficulty medium --tag backtracking
"$LCSRS" add Permutations https://leetcode.com/problems/permutations --difficulty medium --tag backtracking
"$LCSRS" add 'Subsets II' https://leetcode.com/problems/subsets-ii --difficulty medium --tag backtracking
"$LCSRS" add 'Combination Sum II' https://leetcode.com/problems/combination-sum-ii --difficulty medium --tag backtracking
"$LCSRS" add 'Word Search' https://leetcode.com/problems/word-search --difficulty medium --tag backtracking
"$LCSRS" add 'Palindrome Partitioning' https://leetcode.com/problems/palindrome-partitioning --difficulty medium --tag backtracking
"$LCSRS" add 'Letter Combinations of a Phone Number' https://leetcode.com/problems/letter-combinations-of-a-phone-number --difficulty medium --tag backtracking
"$LCSRS" add 'N Queens' https://leetcode.com/problems/n-queens --difficulty hard --tag backtracking
# --- Graphs ---
"$LCSRS" add 'Number of Islands' https://leetcode.com/problems/number-of-islands --difficulty medium --tag graph
"$LCSRS" add 'Clone Graph' https://leetcode.com/problems/clone-graph --difficulty medium --tag graph
"$LCSRS" add 'Max Area of Island' https://leetcode.com/problems/max-area-of-island --difficulty medium --tag graph
"$LCSRS" add 'Pacific Atlantic Water Flow' https://leetcode.com/problems/pacific-atlantic-water-flow --difficulty medium --tag graph
"$LCSRS" add 'Surrounded Regions' https://leetcode.com/problems/surrounded-regions --difficulty medium --tag graph
"$LCSRS" add 'Rotting Oranges' https://leetcode.com/problems/rotting-oranges --difficulty medium --tag graph
"$LCSRS" add 'Walls And Gates' https://leetcode.com/problems/walls-and-gates --difficulty medium --tag graph
"$LCSRS" add 'Course Schedule' https://leetcode.com/problems/course-schedule --difficulty medium --tag graph
"$LCSRS" add 'Course Schedule II' https://leetcode.com/problems/course-schedule-ii --difficulty medium --tag graph
"$LCSRS" add 'Redundant Connection' https://leetcode.com/problems/redundant-connection --difficulty medium --tag graph
"$LCSRS" add 'Number of Connected Components In An Undirected Graph' https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph --difficulty medium --tag graph
"$LCSRS" add 'Graph Valid Tree' https://leetcode.com/problems/graph-valid-tree --difficulty medium --tag graph
"$LCSRS" add 'Word Ladder' https://leetcode.com/problems/word-ladder --difficulty hard --tag graph
# --- Advanced Graphs ---
"$LCSRS" add 'Reconstruct Itinerary' https://leetcode.com/problems/reconstruct-itinerary --difficulty hard --tag advanced-graph
"$LCSRS" add 'Min Cost to Connect All Points' https://leetcode.com/problems/min-cost-to-connect-all-points --difficulty medium --tag advanced-graph
"$LCSRS" add 'Network Delay Time' https://leetcode.com/problems/network-delay-time --difficulty medium --tag advanced-graph
"$LCSRS" add 'Swim In Rising Water' https://leetcode.com/problems/swim-in-rising-water --difficulty hard --tag advanced-graph
"$LCSRS" add 'Alien Dictionary' https://leetcode.com/problems/alien-dictionary --difficulty hard --tag advanced-graph
"$LCSRS" add 'Cheapest Flights Within K Stops' https://leetcode.com/problems/cheapest-flights-within-k-stops --difficulty medium --tag advanced-graph
# --- 1-D Dynamic Programming ---
"$LCSRS" add 'Climbing Stairs' https://leetcode.com/problems/climbing-stairs --difficulty easy --tag dp --tag 1d
"$LCSRS" add 'Min Cost Climbing Stairs' https://leetcode.com/problems/min-cost-climbing-stairs --difficulty easy --tag dp --tag 1d
"$LCSRS" add 'House Robber' https://leetcode.com/problems/house-robber --difficulty medium --tag dp --tag 1d
"$LCSRS" add 'House Robber II' https://leetcode.com/problems/house-robber-ii --difficulty medium --tag dp --tag 1d
"$LCSRS" add 'Longest Palindromic Substring' https://leetcode.com/problems/longest-palindromic-substring --difficulty medium --tag dp --tag 1d
"$LCSRS" add 'Palindromic Substrings' https://leetcode.com/problems/palindromic-substrings --difficulty medium --tag dp --tag 1d
"$LCSRS" add 'Decode Ways' https://leetcode.com/problems/decode-ways --difficulty medium --tag dp --tag 1d
"$LCSRS" add 'Coin Change' https://leetcode.com/problems/coin-change --difficulty medium --tag dp --tag 1d
"$LCSRS" add 'Maximum Product Subarray' https://leetcode.com/problems/maximum-product-subarray --difficulty medium --tag dp --tag 1d
"$LCSRS" add 'Word Break' https://leetcode.com/problems/word-break --difficulty medium --tag dp --tag 1d
"$LCSRS" add 'Longest Increasing Subsequence' https://leetcode.com/problems/longest-increasing-subsequence --difficulty medium --tag dp --tag 1d
"$LCSRS" add 'Partition Equal Subset Sum' https://leetcode.com/problems/partition-equal-subset-sum --difficulty medium --tag dp --tag 1d
# --- 2-D Dynamic Programming ---
"$LCSRS" add 'Unique Paths' https://leetcode.com/problems/unique-paths --difficulty medium --tag dp --tag 2d
"$LCSRS" add 'Longest Common Subsequence' https://leetcode.com/problems/longest-common-subsequence --difficulty medium --tag dp --tag 2d
"$LCSRS" add 'Best Time to Buy And Sell Stock With Cooldown' https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown --difficulty medium --tag dp --tag 2d
"$LCSRS" add 'Coin Change II' https://leetcode.com/problems/coin-change-ii --difficulty medium --tag dp --tag 2d
"$LCSRS" add 'Target Sum' https://leetcode.com/problems/target-sum --difficulty medium --tag dp --tag 2d
"$LCSRS" add 'Interleaving String' https://leetcode.com/problems/interleaving-string --difficulty medium --tag dp --tag 2d
"$LCSRS" add 'Longest Increasing Path In a Matrix' https://leetcode.com/problems/longest-increasing-path-in-a-matrix --difficulty hard --tag dp --tag 2d
"$LCSRS" add 'Distinct Subsequences' https://leetcode.com/problems/distinct-subsequences --difficulty hard --tag dp --tag 2d
"$LCSRS" add 'Edit Distance' https://leetcode.com/problems/edit-distance --difficulty medium --tag dp --tag 2d
"$LCSRS" add 'Burst Balloons' https://leetcode.com/problems/burst-balloons --difficulty hard --tag dp --tag 2d
"$LCSRS" add 'Regular Expression Matching' https://leetcode.com/problems/regular-expression-matching --difficulty hard --tag dp --tag 2d
# --- Greedy ---
"$LCSRS" add 'Maximum Subarray' https://leetcode.com/problems/maximum-subarray --difficulty medium --tag greedy
"$LCSRS" add 'Jump Game' https://leetcode.com/problems/jump-game --difficulty medium --tag greedy
"$LCSRS" add 'Jump Game II' https://leetcode.com/problems/jump-game-ii --difficulty medium --tag greedy
"$LCSRS" add 'Gas Station' https://leetcode.com/problems/gas-station --difficulty medium --tag greedy
"$LCSRS" add 'Hand of Straights' https://leetcode.com/problems/hand-of-straights --difficulty medium --tag greedy
"$LCSRS" add 'Merge Triplets to Form Target Triplet' https://leetcode.com/problems/merge-triplets-to-form-target-triplet --difficulty medium --tag greedy
"$LCSRS" add 'Partition Labels' https://leetcode.com/problems/partition-labels --difficulty medium --tag greedy
"$LCSRS" add 'Valid Parenthesis String' https://leetcode.com/problems/valid-parenthesis-string --difficulty medium --tag greedy
# --- Intervals ---
"$LCSRS" add 'Insert Interval' https://leetcode.com/problems/insert-interval --difficulty medium --tag intervals
"$LCSRS" add 'Merge Intervals' https://leetcode.com/problems/merge-intervals --difficulty medium --tag intervals
"$LCSRS" add 'Non Overlapping Intervals' https://leetcode.com/problems/non-overlapping-intervals --difficulty medium --tag intervals
"$LCSRS" add 'Meeting Rooms' https://leetcode.com/problems/meeting-rooms --difficulty easy --tag intervals
"$LCSRS" add 'Meeting Rooms II' https://leetcode.com/problems/meeting-rooms-ii --difficulty medium --tag intervals
"$LCSRS" add 'Minimum Interval to Include Each Query' https://leetcode.com/problems/minimum-interval-to-include-each-query --difficulty hard --tag intervals
# --- Math & Geometry ---
"$LCSRS" add 'Rotate Image' https://leetcode.com/problems/rotate-image --difficulty medium --tag math --tag geometry
"$LCSRS" add 'Spiral Matrix' https://leetcode.com/problems/spiral-matrix --difficulty medium --tag math --tag geometry
"$LCSRS" add 'Set Matrix Zeroes' https://leetcode.com/problems/set-matrix-zeroes --difficulty medium --tag math --tag geometry
"$LCSRS" add 'Happy Number' https://leetcode.com/problems/happy-number --difficulty easy --tag math --tag geometry
"$LCSRS" add 'Plus One' https://leetcode.com/problems/plus-one --difficulty easy --tag math --tag geometry
"$LCSRS" add 'Pow(x, n)' https://leetcode.com/problems/powx-n --difficulty medium --tag math --tag geometry
"$LCSRS" add 'Multiply Strings' https://leetcode.com/problems/multiply-strings --difficulty medium --tag math --tag geometry
"$LCSRS" add 'Detect Squares' https://leetcode.com/problems/detect-squares --difficulty medium --tag math --tag geometry
# --- Bit Manipulation ---
"$LCSRS" add 'Single Number' https://leetcode.com/problems/single-number --difficulty easy --tag bit-manipulation
"$LCSRS" add 'Number of 1 Bits' https://leetcode.com/problems/number-of-1-bits --difficulty easy --tag bit-manipulation
"$LCSRS" add 'Counting Bits' https://leetcode.com/problems/counting-bits --difficulty easy --tag bit-manipulation
"$LCSRS" add 'Reverse Bits' https://leetcode.com/problems/reverse-bits --difficulty easy --tag bit-manipulation
"$LCSRS" add 'Missing Number' https://leetcode.com/problems/missing-number --difficulty easy --tag bit-manipulation
"$LCSRS" add 'Sum of Two Integers' https://leetcode.com/problems/sum-of-two-integers --difficulty medium --tag bit-manipulation
"$LCSRS" add 'Reverse Integer' https://leetcode.com/problems/reverse-integer --difficulty medium --tag bit-manipulation
