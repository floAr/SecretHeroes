using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class AddCardToken : MonoBehaviour
{
    public GameObject templateCard;
    GameObject g;
    [SerializeField] Transform CardsListUI;

    public void addCardToken()
    {
        g = Instantiate(templateCard, CardsListUI);

        // DestroyImmediate(templateCard, true);
        // Debug.Log("Card added.");
    }

}
