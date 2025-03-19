(function() {
    var type_impls = Object.fromEntries([["rbfrt",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ToBytes-for-u32\" class=\"impl\"><a class=\"src rightside\" href=\"src/rbfrt/table/to_bytes.rs.html#79-83\">Source</a><a href=\"#impl-ToBytes-for-u32\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"rbfrt/table/trait.ToBytes.html\" title=\"trait rbfrt::table::ToBytes\">ToBytes</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.u32.html\">u32</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.to_bytes\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rbfrt/table/to_bytes.rs.html#80-82\">Source</a><a href=\"#method.to_bytes\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"rbfrt/table/trait.ToBytes.html#tymethod.to_bytes\" class=\"fn\">to_bytes</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.u8.html\">u8</a>&gt;</h4></section><section id=\"method.to_u32\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rbfrt/table/to_bytes.rs.html#28-30\">Source</a><a href=\"#method.to_u32\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"rbfrt/table/trait.ToBytes.html#method.to_u32\" class=\"fn\">to_u32</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.u32.html\">u32</a></h4></section><section id=\"method.to_u64\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rbfrt/table/to_bytes.rs.html#32-34\">Source</a><a href=\"#method.to_u64\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"rbfrt/table/trait.ToBytes.html#method.to_u64\" class=\"fn\">to_u64</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.u64.html\">u64</a></h4></section><section id=\"method.to_u128\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rbfrt/table/to_bytes.rs.html#35-37\">Source</a><a href=\"#method.to_u128\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"rbfrt/table/trait.ToBytes.html#method.to_u128\" class=\"fn\">to_u128</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.u128.html\">u128</a></h4></section><section id=\"method.to_string\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rbfrt/table/to_bytes.rs.html#38-40\">Source</a><a href=\"#method.to_string\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"rbfrt/table/trait.ToBytes.html#method.to_string\" class=\"fn\">to_string</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a></h4></section><section id=\"method.to_bool\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rbfrt/table/to_bytes.rs.html#41-43\">Source</a><a href=\"#method.to_bool\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"rbfrt/table/trait.ToBytes.html#method.to_bool\" class=\"fn\">to_bool</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.bool.html\">bool</a></h4></section><section id=\"method.to_ipv4\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rbfrt/table/to_bytes.rs.html#44-46\">Source</a><a href=\"#method.to_ipv4\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"rbfrt/table/trait.ToBytes.html#method.to_ipv4\" class=\"fn\">to_ipv4</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/core/net/ip_addr/struct.Ipv4Addr.html\" title=\"struct core::net::ip_addr::Ipv4Addr\">Ipv4Addr</a>, <a class=\"enum\" href=\"rbfrt/error/enum.RBFRTError.html\" title=\"enum rbfrt::error::RBFRTError\">RBFRTError</a>&gt;</h4></section><section id=\"method.to_ipv6\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rbfrt/table/to_bytes.rs.html#47-49\">Source</a><a href=\"#method.to_ipv6\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"rbfrt/table/trait.ToBytes.html#method.to_ipv6\" class=\"fn\">to_ipv6</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/core/net/ip_addr/struct.Ipv6Addr.html\" title=\"struct core::net::ip_addr::Ipv6Addr\">Ipv6Addr</a>, <a class=\"enum\" href=\"rbfrt/error/enum.RBFRTError.html\" title=\"enum rbfrt::error::RBFRTError\">RBFRTError</a>&gt;</h4></section><section id=\"method.to_int_arr\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rbfrt/table/to_bytes.rs.html#50-52\">Source</a><a href=\"#method.to_int_arr\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"rbfrt/table/trait.ToBytes.html#method.to_int_arr\" class=\"fn\">to_int_arr</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.u32.html\">u32</a>&gt;</h4></section></div></details>","ToBytes","rbfrt::register::IndexType"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[5505]}