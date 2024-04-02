# CSES PROBLEM SET
#### This is the collection of blazingly fast solutions of all problems from [**The CSES Problem Set**](https://cses.fi/problemset/)
> ### **`Note: The repository is currently in progress`**

# Contribution
- **`Template:`** If you use a template which is faster than this [*template*](https://github.com/sumit-ftr/cses-300/blob/master/template.rs), then you can post your template in the *templates/* directory.
- **`Solution:`** Write the solution with this [*template*](https://github.com/sumit-ftr/cses-300/blob/master/template.rs). If you want to write solution with some other template then please check template contribution.

# How to setup scrape?
#### Put your data accordingly
- **`Username`** : `/scrape/userdata/username`
- **`Password`** : `/scrape/userdata/password`
- **`Cookie`** : `/scrape/userdata/cookie`

#### How to get **`CSES`** cookie?
- Go to [https://cses.fi/login](https://cses.fi/login) (Login if you're not logged in)
- Go to inspect mode (Shortcut: Ctrl + Shift + I)
- Go to console and write the following command:
```
document.cookie
```

# How to scrape for best solutions?
- Go to your preferred problem url. Copy the `4` digit number (problem rating) present in the url.
- Clone the repository
```
git clone --depth=1 https://github.com/sumit-ftr/cses-problem-set.git
``` 
- Enter the scrape directory
```
cd scrape
```
- Run scrape
```
cargo run --release <problem_rating>
```

