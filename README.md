# CSES PROBLEM SET
#### This is the collection of blazingly fast solutions of all problems from [**The CSES Problem Set**](https://cses.fi/problemset/)
> ### **`Note: The repository is currently in progress`**

# Contribution
> **Write solution with this [*template*](https://github.com/sumit-ftr/cses-300/blob/master/template.rs). If you want to use some other template then you can create an issue mentioning your template is faster than this one.**

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
- Clone the repository
```
git clone --depth=1 https://github.com/sumit-ftr/cses-problem-set.git
``` 
- Go to your preferred problem url. Copy the `4` digit number (problem rating) present in the url.
- Enter the scrape directory
```
cd cses-problem-set/scrape
```
- Run scrape
```
cargo run --release <problem_rating>
```
