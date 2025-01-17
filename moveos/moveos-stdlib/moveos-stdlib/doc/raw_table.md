
<a name="0x2_raw_table"></a>

# Module `0x2::raw_table`

Raw Key Value table. This is the basic of storage abstraction.
This type table doesn't care about the key and value types. We leave the data type checking to the Native implementation.
This type table is for internal global storage, so all functions are friend.


-  [Resource `TableInfo`](#0x2_raw_table_TableInfo)
-  [Resource `Box`](#0x2_raw_table_Box)
-  [Constants](#@Constants_0)
-  [Function `global_object_storage_handle`](#0x2_raw_table_global_object_storage_handle)
-  [Function `add`](#0x2_raw_table_add)
-  [Function `borrow`](#0x2_raw_table_borrow)
-  [Function `borrow_from_global`](#0x2_raw_table_borrow_from_global)
-  [Function `borrow_with_default`](#0x2_raw_table_borrow_with_default)
-  [Function `borrow_mut`](#0x2_raw_table_borrow_mut)
-  [Function `borrow_mut_from_global`](#0x2_raw_table_borrow_mut_from_global)
-  [Function `borrow_mut_with_default`](#0x2_raw_table_borrow_mut_with_default)
-  [Function `upsert`](#0x2_raw_table_upsert)
-  [Function `remove`](#0x2_raw_table_remove)
-  [Function `remove_from_global`](#0x2_raw_table_remove_from_global)
-  [Function `contains`](#0x2_raw_table_contains)
-  [Function `contains_global`](#0x2_raw_table_contains_global)
-  [Function `length`](#0x2_raw_table_length)
-  [Function `is_empty`](#0x2_raw_table_is_empty)
-  [Function `drop_unchecked`](#0x2_raw_table_drop_unchecked)
-  [Function `destroy_empty`](#0x2_raw_table_destroy_empty)
-  [Function `new_table_handle`](#0x2_raw_table_new_table_handle)


<pre><code><b>use</b> <a href="object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x2_raw_table_TableInfo"></a>

## Resource `TableInfo`



<pre><code><b>struct</b> <a href="raw_table.md#0x2_raw_table_TableInfo">TableInfo</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>state_root: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>size: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x2_raw_table_Box"></a>

## Resource `Box`

Wrapper for values. Required for making values appear as resources in the implementation.
Because the GlobalValue in MoveVM must be a resource.


<pre><code><b>struct</b> <a href="raw_table.md#0x2_raw_table_Box">Box</a>&lt;V&gt; <b>has</b> drop, store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>val: V</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x2_raw_table_ErrorAlreadyExists"></a>

The key already exists in the table


<pre><code><b>const</b> <a href="raw_table.md#0x2_raw_table_ErrorAlreadyExists">ErrorAlreadyExists</a>: u64 = 1;
</code></pre>



<a name="0x2_raw_table_ErrorDuplicateOperation"></a>

Duplicate operation on the table


<pre><code><b>const</b> <a href="raw_table.md#0x2_raw_table_ErrorDuplicateOperation">ErrorDuplicateOperation</a>: u64 = 3;
</code></pre>



<a name="0x2_raw_table_ErrorNotEmpty"></a>

The table is not empty


<pre><code><b>const</b> <a href="raw_table.md#0x2_raw_table_ErrorNotEmpty">ErrorNotEmpty</a>: u64 = 4;
</code></pre>



<a name="0x2_raw_table_ErrorNotFound"></a>

Can not found the key in the table


<pre><code><b>const</b> <a href="raw_table.md#0x2_raw_table_ErrorNotFound">ErrorNotFound</a>: u64 = 2;
</code></pre>



<a name="0x2_raw_table_GlobalObjectStorageHandle"></a>



<pre><code><b>const</b> <a href="raw_table.md#0x2_raw_table_GlobalObjectStorageHandle">GlobalObjectStorageHandle</a>: <b>address</b> = 0;
</code></pre>



<a name="0x2_raw_table_global_object_storage_handle"></a>

## Function `global_object_storage_handle`

The global object storage's table handle should be <code>0x0</code>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_global_object_storage_handle">global_object_storage_handle</a>(): <a href="object.md#0x2_object_ObjectID">object::ObjectID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_global_object_storage_handle">global_object_storage_handle</a>(): ObjectID {
    <a href="object.md#0x2_object_address_to_object_id">object::address_to_object_id</a>(<a href="raw_table.md#0x2_raw_table_GlobalObjectStorageHandle">GlobalObjectStorageHandle</a>)
}
</code></pre>



</details>

<a name="0x2_raw_table_add"></a>

## Function `add`

Add a new entry to the table. Aborts if an entry for this
key already exists. The entry itself is not stored in the
table, and cannot be discovered from it.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_add">add</a>&lt;K: <b>copy</b>, drop, V&gt;(table_handle: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>, key: K, val: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_add">add</a>&lt;K: <b>copy</b> + drop, V&gt;(table_handle: &ObjectID, key: K, val: V) {
    <a href="raw_table.md#0x2_raw_table_add_box">add_box</a>&lt;K, V, <a href="raw_table.md#0x2_raw_table_Box">Box</a>&lt;V&gt;&gt;(*table_handle, key, <a href="raw_table.md#0x2_raw_table_Box">Box</a> {val} );
}
</code></pre>



</details>

<a name="0x2_raw_table_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the value which <code>key</code> maps to.
Aborts if there is no entry for <code>key</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_borrow">borrow</a>&lt;K: <b>copy</b>, drop, V&gt;(table_handle: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>, key: K): &V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_borrow">borrow</a>&lt;K: <b>copy</b> + drop, V&gt;(table_handle: &ObjectID, key: K): &V {
    &<a href="raw_table.md#0x2_raw_table_borrow_box">borrow_box</a>&lt;K, V, <a href="raw_table.md#0x2_raw_table_Box">Box</a>&lt;V&gt;&gt;(*table_handle, key).val
}
</code></pre>



</details>

<a name="0x2_raw_table_borrow_from_global"></a>

## Function `borrow_from_global`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_borrow_from_global">borrow_from_global</a>&lt;T: key&gt;(object_id: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>): &<a href="object.md#0x2_object_Object">object::Object</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_borrow_from_global">borrow_from_global</a>&lt;T: key&gt;(object_id: &ObjectID): &Object&lt;T&gt; {
    &<a href="raw_table.md#0x2_raw_table_borrow_box">borrow_box</a>&lt;ObjectID, Object&lt;T&gt;, <a href="raw_table.md#0x2_raw_table_Box">Box</a>&lt;Object&lt;T&gt;&gt;&gt;(<a href="raw_table.md#0x2_raw_table_global_object_storage_handle">global_object_storage_handle</a>(), *object_id).val
}
</code></pre>



</details>

<a name="0x2_raw_table_borrow_with_default"></a>

## Function `borrow_with_default`

Acquire an immutable reference to the value which <code>key</code> maps to.
Returns specified default value if there is no entry for <code>key</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_borrow_with_default">borrow_with_default</a>&lt;K: <b>copy</b>, drop, V&gt;(table_handle: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>, key: K, default: &V): &V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_borrow_with_default">borrow_with_default</a>&lt;K: <b>copy</b> + drop, V&gt;(table_handle: &ObjectID, key: K, default: &V): &V {
    <b>if</b> (!<a href="raw_table.md#0x2_raw_table_contains">contains</a>&lt;K&gt;(table_handle, key)) {
        default
    } <b>else</b> {
        <a href="raw_table.md#0x2_raw_table_borrow">borrow</a>(table_handle, key)
    }
}
</code></pre>



</details>

<a name="0x2_raw_table_borrow_mut"></a>

## Function `borrow_mut`

Acquire a mutable reference to the value which <code>key</code> maps to.
Aborts if there is no entry for <code>key</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_borrow_mut">borrow_mut</a>&lt;K: <b>copy</b>, drop, V&gt;(table_handle: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>, key: K): &<b>mut</b> V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_borrow_mut">borrow_mut</a>&lt;K: <b>copy</b> + drop, V&gt;(table_handle: &ObjectID, key: K): &<b>mut</b> V {
    &<b>mut</b> <a href="raw_table.md#0x2_raw_table_borrow_box_mut">borrow_box_mut</a>&lt;K, V, <a href="raw_table.md#0x2_raw_table_Box">Box</a>&lt;V&gt;&gt;(*table_handle, key).val
}
</code></pre>



</details>

<a name="0x2_raw_table_borrow_mut_from_global"></a>

## Function `borrow_mut_from_global`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_borrow_mut_from_global">borrow_mut_from_global</a>&lt;T: key&gt;(object_id: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>): &<b>mut</b> <a href="object.md#0x2_object_Object">object::Object</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_borrow_mut_from_global">borrow_mut_from_global</a>&lt;T: key&gt;(object_id: &ObjectID): &<b>mut</b> Object&lt;T&gt; {
    &<b>mut</b> <a href="raw_table.md#0x2_raw_table_borrow_box_mut">borrow_box_mut</a>&lt;ObjectID, Object&lt;T&gt;, <a href="raw_table.md#0x2_raw_table_Box">Box</a>&lt;Object&lt;T&gt;&gt;&gt;(<a href="raw_table.md#0x2_raw_table_global_object_storage_handle">global_object_storage_handle</a>(), *object_id).val
}
</code></pre>



</details>

<a name="0x2_raw_table_borrow_mut_with_default"></a>

## Function `borrow_mut_with_default`

Acquire a mutable reference to the value which <code>key</code> maps to.
Insert the pair (<code>key</code>, <code>default</code>) first if there is no entry for <code>key</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_borrow_mut_with_default">borrow_mut_with_default</a>&lt;K: <b>copy</b>, drop, V: drop&gt;(table_handle: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>, key: K, default: V): &<b>mut</b> V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_borrow_mut_with_default">borrow_mut_with_default</a>&lt;K: <b>copy</b> + drop, V: drop&gt;(table_handle: &ObjectID, key: K, default: V): &<b>mut</b> V {
    <b>if</b> (!<a href="raw_table.md#0x2_raw_table_contains">contains</a>&lt;K&gt;(table_handle, <b>copy</b> key)) {
        <a href="raw_table.md#0x2_raw_table_add">add</a>(table_handle, key, default)
    };
    <a href="raw_table.md#0x2_raw_table_borrow_mut">borrow_mut</a>(table_handle, key)
}
</code></pre>



</details>

<a name="0x2_raw_table_upsert"></a>

## Function `upsert`

Insert the pair (<code>key</code>, <code>value</code>) if there is no entry for <code>key</code>.
update the value of the entry for <code>key</code> to <code>value</code> otherwise


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_upsert">upsert</a>&lt;K: <b>copy</b>, drop, V: drop&gt;(table_handle: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>, key: K, value: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_upsert">upsert</a>&lt;K: <b>copy</b> + drop, V: drop&gt;(table_handle: &ObjectID, key: K, value: V) {
    <b>if</b> (!<a href="raw_table.md#0x2_raw_table_contains">contains</a>&lt;K&gt;(table_handle, <b>copy</b> key)) {
        <a href="raw_table.md#0x2_raw_table_add">add</a>(table_handle, key, value)
    } <b>else</b> {
        <b>let</b> ref = <a href="raw_table.md#0x2_raw_table_borrow_mut">borrow_mut</a>(table_handle, key);
        *ref = value;
    };
}
</code></pre>



</details>

<a name="0x2_raw_table_remove"></a>

## Function `remove`

Remove from <code><a href="table.md#0x2_table">table</a></code> and return the value which <code>key</code> maps to.
Aborts if there is no entry for <code>key</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_remove">remove</a>&lt;K: <b>copy</b>, drop, V&gt;(table_handle: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>, key: K): V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_remove">remove</a>&lt;K: <b>copy</b> + drop, V&gt;(table_handle: &ObjectID, key: K): V {
    <b>let</b> <a href="raw_table.md#0x2_raw_table_Box">Box</a> { val } = <a href="raw_table.md#0x2_raw_table_remove_box">remove_box</a>&lt;K, V, <a href="raw_table.md#0x2_raw_table_Box">Box</a>&lt;V&gt;&gt;(*table_handle, key);
    val
}
</code></pre>



</details>

<a name="0x2_raw_table_remove_from_global"></a>

## Function `remove_from_global`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_remove_from_global">remove_from_global</a>&lt;T: key&gt;(object_id: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>): <a href="object.md#0x2_object_Object">object::Object</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_remove_from_global">remove_from_global</a>&lt;T: key&gt;(object_id: &ObjectID): Object&lt;T&gt; {
    <b>let</b> <a href="raw_table.md#0x2_raw_table_Box">Box</a> { val } = <a href="raw_table.md#0x2_raw_table_remove_box">remove_box</a>&lt;ObjectID, Object&lt;T&gt;, <a href="raw_table.md#0x2_raw_table_Box">Box</a>&lt;Object&lt;T&gt;&gt;&gt;(<a href="raw_table.md#0x2_raw_table_global_object_storage_handle">global_object_storage_handle</a>(), *object_id);
    val
}
</code></pre>



</details>

<a name="0x2_raw_table_contains"></a>

## Function `contains`

Returns true if <code><a href="table.md#0x2_table">table</a></code> contains an entry for <code>key</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_contains">contains</a>&lt;K: <b>copy</b>, drop&gt;(table_handle: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>, key: K): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_contains">contains</a>&lt;K: <b>copy</b> + drop&gt;(table_handle: &ObjectID, key: K): bool {
    <a href="raw_table.md#0x2_raw_table_contains_box">contains_box</a>&lt;K&gt;(*table_handle, key)
}
</code></pre>



</details>

<a name="0x2_raw_table_contains_global"></a>

## Function `contains_global`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_contains_global">contains_global</a>(object_id: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_contains_global">contains_global</a>(object_id: &ObjectID): bool {
    <a href="raw_table.md#0x2_raw_table_contains_box">contains_box</a>&lt;ObjectID&gt;(<a href="raw_table.md#0x2_raw_table_global_object_storage_handle">global_object_storage_handle</a>(), *object_id)
}
</code></pre>



</details>

<a name="0x2_raw_table_length"></a>

## Function `length`

Returns the size of the table, the number of key-value pairs


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_length">length</a>(table_handle: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_length">length</a>(table_handle: &ObjectID): u64 {
    <a href="raw_table.md#0x2_raw_table_box_length">box_length</a>(*table_handle)
}
</code></pre>



</details>

<a name="0x2_raw_table_is_empty"></a>

## Function `is_empty`

Returns true if the table is empty (if <code>length</code> returns <code>0</code>)


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_is_empty">is_empty</a>(table_handle: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_is_empty">is_empty</a>(table_handle: &ObjectID): bool {
    <a href="raw_table.md#0x2_raw_table_length">length</a>(table_handle) == 0
}
</code></pre>



</details>

<a name="0x2_raw_table_drop_unchecked"></a>

## Function `drop_unchecked`

Drop a table even if it is not empty.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_drop_unchecked">drop_unchecked</a>(table_handle: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_drop_unchecked">drop_unchecked</a>(table_handle: &ObjectID) {
    <a href="raw_table.md#0x2_raw_table_drop_unchecked_box">drop_unchecked_box</a>(*table_handle)
}
</code></pre>



</details>

<a name="0x2_raw_table_destroy_empty"></a>

## Function `destroy_empty`

Destroy a table. Aborts if the table is not empty


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_destroy_empty">destroy_empty</a>(table_handle: &<a href="object.md#0x2_object_ObjectID">object::ObjectID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_destroy_empty">destroy_empty</a>(table_handle: &ObjectID) {
    <b>assert</b>!(<a href="raw_table.md#0x2_raw_table_is_empty">is_empty</a>(table_handle), <a href="raw_table.md#0x2_raw_table_ErrorNotEmpty">ErrorNotEmpty</a>);
    <a href="raw_table.md#0x2_raw_table_drop_unchecked_box">drop_unchecked_box</a>(*table_handle)
}
</code></pre>



</details>

<a name="0x2_raw_table_new_table_handle"></a>

## Function `new_table_handle`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_new_table_handle">new_table_handle</a>(ctx: &<b>mut</b> <a href="tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="object.md#0x2_object_ObjectID">object::ObjectID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="raw_table.md#0x2_raw_table_new_table_handle">new_table_handle</a>(ctx: &<b>mut</b> TxContext): ObjectID {
    <a href="object.md#0x2_object_address_to_object_id">object::address_to_object_id</a>(<a href="tx_context.md#0x2_tx_context_fresh_address">tx_context::fresh_address</a>(ctx))
}
</code></pre>



</details>
