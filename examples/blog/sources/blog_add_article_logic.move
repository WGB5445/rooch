module rooch_examples::blog_add_article_logic {
    use std::vector;

    use moveos_std::object_id::ObjectID;
    use moveos_std::storage_context::StorageContext;
    use rooch_examples::article_added_to_blog;
    use rooch_examples::blog;

    friend rooch_examples::blog_aggregate;

    public(friend) fun verify(
        storage_ctx: &mut StorageContext,
        account: &signer,
        article_id: ObjectID,
        blog: &blog::Blog,
    ): blog::ArticleAddedToBlog {
        let _ = storage_ctx;
        let _ = account;
        blog::new_article_added_to_blog(
            blog,
            article_id,
        )
    }

    public(friend) fun mutate(
        storage_ctx: &mut StorageContext,
        _account: &signer,
        article_added_to_blog: &blog::ArticleAddedToBlog,
        blog: blog::Blog,
    ): blog::Blog {
        let _ = storage_ctx;
        let article_id = article_added_to_blog::article_id(article_added_to_blog);
        let articles = blog::articles(&blog);
        if (!vector::contains(&articles, &article_id)) {
            vector::push_back(&mut articles, article_id);
            blog::set_articles(&mut blog, articles);
        };
        blog
    }
}