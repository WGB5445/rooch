// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

module rooch_examples::article_create_logic {
    use moveos_std::object_ref::ObjectRef;
    use moveos_std::context::Context;
    use rooch_examples::article::{Self, Article};
    use rooch_examples::article_created;
    use std::string::String;
    use rooch_examples::blog_aggregate;

    friend rooch_examples::article_aggregate;

    public(friend) fun verify(
        ctx: &mut Context,
        account: &signer,
        title: String,
        body: String,
    ): article::ArticleCreated {
        let _ = ctx;
        let _ = account;
        article::new_article_created(
            title,
            body,
        )
    }

    public(friend) fun mutate(
        ctx: &mut Context,
        _account: &signer,
        article_created: &article::ArticleCreated,
    ): ObjectRef<Article> {
        let title = article_created::title(article_created);
        let body = article_created::body(article_created);
        let article_obj = article::create_article(
            ctx,
            title,
            body,
        );
        // ///////////////////////////
        blog_aggregate::add_article(ctx, article::id(&article_obj));
        // ///////////////////////////
        article_obj
    }

}
