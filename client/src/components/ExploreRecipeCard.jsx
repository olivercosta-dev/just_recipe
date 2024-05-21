export function ExploreRecipeCard({ image, name, description }) {
    return (
        <div class="explore-card">
            <img class="explore-card-image" src={image} />
            <div class="explore-card-info">
                <span class="explore-card-name">{name}</span>
                <span class="explore-card-description">
                    {description}
                </span>
            </div>
        </div>
    )
}