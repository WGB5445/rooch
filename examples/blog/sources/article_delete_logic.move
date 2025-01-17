// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

module rooch_examples::article_delete_logic {
    use moveos_std::object_ref::ObjectRef;
    use moveos_std::context::Context;
    use rooch_examples::article::{Self, Article};
    use rooch_examples::blog_aggregate;

    friend rooch_examples::article_aggregate;

    public(friend) fun verify(
        ctx: &mut Context,
        account: &signer,
        article_obj: &ObjectRef<Article>,
    ): article::ArticleDeleted {
        let _ = ctx;
        let _ = account;
        article::new_article_deleted(
            article_obj,
        )
    }

    public(friend) fun mutate(
        ctx: &mut Context,
        _account: &signer,
        article_deleted: &article::ArticleDeleted,
        article_obj: ObjectRef<Article>,
    ): ObjectRef<Article> {
        let _ = article_deleted;
        blog_aggregate::remove_article(ctx, article::id(&article_obj));
        article_obj
    }

}
