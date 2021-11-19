<<<<<<< Updated upstream
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PopulateGrid : MonoBehaviour
{
    public GameObject prefab;
    public int numberToCreate;
    //AR added
    public List<Token> MyHeroes;
    //

=======
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PopulateGrid : MonoBehaviour
{
    public GameObject prefab;
    public int numberToCreate;
    //AR added
    public List<Token> MyHeroes;
    //

>>>>>>> Stashed changes
    [ContextMenu("TST ADD")]
    public void AddToken()
    {
        int i = MyHeroes.Count;
        MyHeroes.Add(new Token() { biotech = i, engineering = i, psychics = i, weapons = i, name = "" });
<<<<<<< Updated upstream
    }

    void Start()
    {
        Populate();

   //     for (int i = 0; i < 3; i++)
  //      {
    //        AddToken();
    //    }
    }

    // Update is called once per frame
    void Update()
    {
        
    }

=======
    }

    void Start()
    {
        Populate();
    }

    // Update is called once per frame
    void Update()
    {

    }

>>>>>>> Stashed changes
    public void Populate()
    {
        GameObject newObj;
        List<GameObject> listOfHeroGameObjects = new List<GameObject>();
        GameObject[] arrayOfGameObjects = listOfHeroGameObjects.ToArray();
        //AR no longer use number to create 
        for (int i = 0; i < MyHeroes.Count; i++)
<<<<<<< Updated upstream


=======
>>>>>>> Stashed changes
        {
            //AddToken();
            // AR REMOVED this 43 to add new tokens
            newObj = (GameObject)Instantiate(prefab, transform);
<<<<<<< Updated upstream
          

        }
    }
=======


        }
    }
>>>>>>> Stashed changes
    internal void UpdateTokens(Token[] tokens)
    {
        foreach (var item in tokens)
        {
            var hero = MyHeroes.Find(t => { return t.id.Equals(item.id); });
            if (hero == null)
            {
                MyHeroes.Add(item);
            }
            else
            {
                hero.weapons = item.weapons;
                hero.engineering = item.engineering;
                hero.biotech = item.biotech;
                hero.psychics = item.psychics;
            }
        }

        MyHeroes.Clear();
        MyHeroes.AddRange(tokens);
    }
<<<<<<< Updated upstream
    }
=======
}
>>>>>>> Stashed changes
