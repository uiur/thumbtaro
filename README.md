# thumbtaro
A thumbnail server written in Rust. It supports Google Cloud Storage as storage backend.

## API
Suppose food/banana.png exists under configured bucket.
### GET /orig/:path
Example: `/orig/food/banana.png`

This endpoint returns an original image.

### GET /thumb/{width}x{height}/:path
Example: `/thumb/400x400/food/banana.png`

This endpoint returns a thumbnail with specified width and height generated from an original image.

## Configuration

env:

```
# required
GOOGLE_APPLICATION_CREDENTIALS=~/google_credentials.json
THUMBTARO_BUCKET=bucket

# optional
THUMBTARO_KEY_PREFIX=uploads/
```
