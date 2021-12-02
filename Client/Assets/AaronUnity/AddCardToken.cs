using System;
using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

public static class ButtonExtension
{
    public static void addEventListener<T>(this Button button, T param, Action<T> OnClick)
    {
        button.onClick.AddListener(delegate ()
        {
            OnClick(param);
        });
    }
}
public class AddCardToken : MonoBehaviour
{
    public CameraController cameraController;
    public GameObject templateCard;
    GameObject g;
    [SerializeField] Transform CardsListUI;

    [SerializeField] GameObject imageHolder;
    [SerializeField] GameObject statsHolder;

    [SerializeField] GameObject OptionButtonsHolder;
    [SerializeField] GameObject UpgradeHeroesHolder;
    [SerializeField] GameObject OddsBurnHolder;

    [SerializeField] GameObject OddsBurnHolder_fieds;

    [SerializeField] GameObject btnCancel;

    [SerializeField] Sprite overrideImage;
    [SerializeField] TextMeshProUGUI TopTitle;

    private List<Token> myHeores;

    public CardRenderer CharacterRenderer;

    private int i;
    private int selectedindex = 0;
    private string selectionState;

    private int randomUpgradeToken = 0;

    private List<string> selectedBoth;

    private CardDisplay tempGameObject;

    private string pathFolder = "";
    private string prefabName = "CardRenderer";

    public ParticleSystem fx;

    public GameObject thumbnailCreator;

    void Awake()
    {
        StartCoroutine(InitializeGrid());
    }

    void Start()
    {
        if (GameObject.FindObjectOfType<ThumbnailSetup>().selectCustomFolder)
            pathFolder = FindObjectOfType<ThumbnailSetup>().FolderFullPathName();
        else
            pathFolder = Application.dataPath;
    }

    void Update()
    {
        if (selectedBoth.Count == 1)
        {
            TopTitle.text = "Choose One More Hero To Burn";
        }

        else if (selectedBoth.Count == 2)
        {
            TopTitle.text = "Ready To Upgrade";
        }

        if (selectionState.Equals("selection_to_burn"))
        {
            #region Change Fields Color


            //Main Text
            setColorOfFiled02(OddsBurnHolder_fieds.transform.GetChild(4).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(28).GetComponent<TextMeshProUGUI>()));
            setColorOfFiled02(OddsBurnHolder_fieds.transform.GetChild(5).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(29).GetComponent<TextMeshProUGUI>()));
            setColorOfFiled02(OddsBurnHolder_fieds.transform.GetChild(6).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(30).GetComponent<TextMeshProUGUI>()));
            setColorOfFiled02(OddsBurnHolder_fieds.transform.GetChild(7).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(31).GetComponent<TextMeshProUGUI>()));

            setColorOfFiled02(OddsBurnHolder_fieds.transform.GetChild(12).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(36).GetComponent<TextMeshProUGUI>()));
            setColorOfFiled02(OddsBurnHolder_fieds.transform.GetChild(13).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(37).GetComponent<TextMeshProUGUI>()));
            setColorOfFiled02(OddsBurnHolder_fieds.transform.GetChild(14).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(38).GetComponent<TextMeshProUGUI>()));
            setColorOfFiled02(OddsBurnHolder_fieds.transform.GetChild(15).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(39).GetComponent<TextMeshProUGUI>()));

            //Small Text
            setColorOfFiled01(OddsBurnHolder_fieds.transform.GetChild(28).GetComponent<TextMeshProUGUI>());
            setColorOfFiled01(OddsBurnHolder_fieds.transform.GetChild(29).GetComponent<TextMeshProUGUI>());
            setColorOfFiled01(OddsBurnHolder_fieds.transform.GetChild(30).GetComponent<TextMeshProUGUI>());
            setColorOfFiled01(OddsBurnHolder_fieds.transform.GetChild(31).GetComponent<TextMeshProUGUI>());

            setColorOfFiled01(OddsBurnHolder_fieds.transform.GetChild(36).GetComponent<TextMeshProUGUI>());
            setColorOfFiled01(OddsBurnHolder_fieds.transform.GetChild(37).GetComponent<TextMeshProUGUI>());
            setColorOfFiled01(OddsBurnHolder_fieds.transform.GetChild(38).GetComponent<TextMeshProUGUI>());
            setColorOfFiled01(OddsBurnHolder_fieds.transform.GetChild(39).GetComponent<TextMeshProUGUI>());


            #endregion
        }
    }

    void OnEnable()
    {
        cameraController.enabled = false;
        thumbnailCreator.SetActive(true);
    }

<<<<<<< Updated upstream:Client/Assets/AaronUnity/AddCardToken.cs
    public void addCardToken()
=======
    void OnDisable()
    {
        if (cameraController != null) cameraController.enabled = true;
        thumbnailCreator.SetActive(false);
    }

    private IEnumerator InitializeGrid()
    {
        i = 0;
        selectionState = "selection_hero";
        selectedBoth = new List<string>();
        imageHolder.SetActive(true);
        statsHolder.SetActive(false);
        btnCancel.SetActive(false);
        UpgradeHeroesHolder.SetActive(false);
        OptionButtonsHolder.SetActive(false);
        TopTitle.text = "Choose Hero";

        for (int k = 0; k < CardsListUI.transform.childCount; k++)
            Destroy(CardsListUI.transform.GetChild(k).gameObject);

        yield return new WaitForSeconds(1.25f);

        int count = GameObject.FindObjectOfType<Rooster>().MyHeroes.Count;
        if (count > 0)
        {
            foreach (Token currentToken in GameObject.FindObjectOfType<Rooster>().MyHeroes)
            {
                StartCoroutine(readNewCardToken(currentToken));
            }
        }

        thumbnailCreator.SetActive(false);

        yield return true;
    }
    private IEnumerator readNewCardToken(Token currentToken)
>>>>>>> Stashed changes:Client/Assets/AaronUnity/Scripts/AddCardToken.cs
    {
        g = Instantiate(templateCard, CardsListUI);

        CharacterRenderer.ReadToken(currentToken);

        GameObject.FindObjectOfType<ThumbnailSetup>().executeThumbnailCapture();

        g.transform.gameObject.GetComponent<CardDisplay>().updateTokenValues(currentToken.name, currentToken.weapons, currentToken.engineering, currentToken.biotech, currentToken.psychics, currentToken.id, pathFolder + "/" + prefabName + ".png");

        g.transform.GetChild(0).GetComponent<Button>().addEventListener(i, onClickEvent);

        InstantiatedTokens.instantiatedObjects.Add(g);

        i++;

        yield return new WaitForSeconds(2f);
    }

    private void updateNewStats(int a, int b, int c, int d)
    {
        #region update selected card

        statsHolder.transform.GetChild(1).GetChild(0).GetComponentInChildren<TextMeshProUGUI>().text = a.ToString();

        statsHolder.transform.GetChild(1).GetChild(1).GetComponentInChildren<TextMeshProUGUI>().text = b.ToString();

        statsHolder.transform.GetChild(1).GetChild(2).GetComponentInChildren<TextMeshProUGUI>().text = c.ToString();

        statsHolder.transform.GetChild(1).GetChild(3).GetComponentInChildren<TextMeshProUGUI>().text = d.ToString();

        #endregion
    }

    public void addCardToken()
    {
    }

    void onClickEvent(int index)
    {
        printIndex(index);
        selectedindex = index;

        #region Check Selection State
        if (selectionState.Equals("selection_hero"))
        {
            imageHolder.SetActive(false);
            statsHolder.SetActive(true);
            btnCancel.SetActive(true);
            UpgradeHeroesHolder.SetActive(false);
            OptionButtonsHolder.SetActive(true);
<<<<<<< HEAD
=======
            //Added to jump into selection process
            selectionState.Equals("selection_to_burn");
            

        }
        else if (selectionState.Equals("selection_to_burn") && selectedBoth.Count == 0)
        {
            selectedBoth.Add(InstantiatedTokens.instantiatedObjects[selectedindex]);

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().interactable = false;

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().image.overrideSprite = overrideImage;

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.GetComponent<CardDisplay>().InactifCardBurn.SetActive(true);

>>>>>>> parent of 235d4c37 (Cancel Button)
        }
        else if (selectionState.Equals("selection_to_burn") && selectedBoth.Count < 2)
        {
<<<<<<< Updated upstream:Client/Assets/AaronUnity/AddCardToken.cs
            selectedBoth.Add(InstantiatedTokens.instantiatedObjects[selectedindex]);
=======
            selectedBoth.Add(InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.GetComponent<CardDisplay>().txtId.text);
>>>>>>> Stashed changes:Client/Assets/AaronUnity/Scripts/AddCardToken.cs

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().interactable = false;

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().image.overrideSprite = overrideImage;

<<<<<<< Updated upstream:Client/Assets/AaronUnity/AddCardToken.cs
            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.GetComponent<CardDisplay>().InactifCardBurn.SetActive(true);

        }
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
     
        
=======
            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetChild(2).GetComponent<Image>().enabled = true;
        }
>>>>>>> Stashed changes:Client/Assets/AaronUnity/Scripts/AddCardToken.cs
=======
>>>>>>> parent of 235d4c37 (Cancel Button)
=======
>>>>>>> parent of 235d4c37 (Cancel Button)

=======
>>>>>>> parent of ec36a1f2 (CardClick)
        #endregion

        if (!selectionState.Equals("selection_hero")) return;

        tempGameObject = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>();

        #region Update Thumbnail Stats
        statsHolder.transform.GetChild(1).GetChild(0).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtStat1.text;

        statsHolder.transform.GetChild(1).GetChild(1).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtStat2.text;

        statsHolder.transform.GetChild(1).GetChild(2).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtStat3.text;

        statsHolder.transform.GetChild(1).GetChild(3).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtStat4.text;

        statsHolder.transform.GetChild(1).GetChild(4).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().txtName.text;
<<<<<<< Updated upstream:Client/Assets/AaronUnity/AddCardToken.cs
=======

        statsHolder.transform.GetChild(0).GetComponentInChildren<Image>().sprite = InstantiatedTokens.instantiatedObjects[index].gameObject.GetComponent<CardDisplay>().image.sprite;

>>>>>>> Stashed changes:Client/Assets/AaronUnity/Scripts/AddCardToken.cs
        #endregion

    }

    public void onStartBothSelection()
    {
        if (InstantiatedTokens.instantiatedObjects.Count < 3) return;

        selectionState = "selection_to_burn";
        TopTitle.text = "Choose Two Heroes To Burn";

        OptionButtonsHolder.SetActive(false);
        UpgradeHeroesHolder.SetActive(true);

        InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().interactable = false;
    }

    public void onCloseOddBurnPanel()
    {
        OddsBurnHolder.SetActive(false);
<<<<<<< HEAD
    }
=======
        selectionState = "selection_hero";
        //Added to close
        i = 0;
        //Added to close
        selectionState = "selection_hero";
        //Added
       

        InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().interactable = true;

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.transform.GetChild(0).GetComponent<Button>().image.overrideSprite = overrideImage;

            InstantiatedTokens.instantiatedObjects[selectedindex].gameObject.GetComponent<CardDisplay>().InactifCardBurn.SetActive(false);

        }
>>>>>>> parent of 235d4c37 (Cancel Button)

        public void onOpenOddBurnPanel()
    {
        if (selectedBoth.Count == 0) return;

        OddsBurnHolder.SetActive(true);

        #region Fill Selected Main Card
        OddsBurnHolder_fieds.transform.GetChild(0).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(1).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(2).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(3).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion


        #region Fill Green 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(4).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(5).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(6).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(7).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion

        #region Fill White 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(8).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(9).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(10).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(11).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion

        #region Fill Red 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(12).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(13).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(14).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(15).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion


        #region Fill Small Green 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(16).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(17).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(18).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(19).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion

        #region Fill Small White 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(20).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(21).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(22).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(23).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion

        #region Fill Small Red 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(24).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat1.text;
        OddsBurnHolder_fieds.transform.GetChild(25).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat2.text;
        OddsBurnHolder_fieds.transform.GetChild(26).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat3.text;
        OddsBurnHolder_fieds.transform.GetChild(27).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtStat4.text;
        #endregion


        #region Fill Extra Small Green 33% Burned Card
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(28).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(29).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(30).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(31).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, 4));

        updateOldField(OddsBurnHolder_fieds.transform.GetChild(4).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(28).GetComponent<TextMeshProUGUI>().text));
        updateOldField(OddsBurnHolder_fieds.transform.GetChild(5).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(29).GetComponent<TextMeshProUGUI>().text));
        updateOldField(OddsBurnHolder_fieds.transform.GetChild(6).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(30).GetComponent<TextMeshProUGUI>().text)); updateOldField(OddsBurnHolder_fieds.transform.GetChild(7).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(31).GetComponent<TextMeshProUGUI>().text));


        #endregion

        #region Fill Extra Small White 33% Burned Card
        OddsBurnHolder_fieds.transform.GetChild(32).GetComponent<TextMeshProUGUI>().text = "+0";
        OddsBurnHolder_fieds.transform.GetChild(33).GetComponent<TextMeshProUGUI>().text = "+0";
        OddsBurnHolder_fieds.transform.GetChild(34).GetComponent<TextMeshProUGUI>().text = "+0";
        OddsBurnHolder_fieds.transform.GetChild(35).GetComponent<TextMeshProUGUI>().text = "+0";
        #endregion

        #region Fill Extra Small Red 33% Burned Card

        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(36).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(37).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(38).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));
        setSignOfFiled(OddsBurnHolder_fieds.transform.GetChild(39).GetComponent<TextMeshProUGUI>(), UnityEngine.Random.Range(0, -4));

        updateOldField(OddsBurnHolder_fieds.transform.GetChild(12).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(36).GetComponent<TextMeshProUGUI>().text));
        updateOldField(OddsBurnHolder_fieds.transform.GetChild(13).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(37).GetComponent<TextMeshProUGUI>().text));
        updateOldField(OddsBurnHolder_fieds.transform.GetChild(14).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(38).GetComponent<TextMeshProUGUI>().text)); updateOldField(OddsBurnHolder_fieds.transform.GetChild(15).GetComponent<TextMeshProUGUI>(), Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(39).GetComponent<TextMeshProUGUI>().text));


        #endregion


        #region Fill Card Name
        OddsBurnHolder_fieds.transform.GetChild(40).GetComponent<TextMeshProUGUI>().text = tempGameObject.txtName.text;
        #endregion

    }

    private void printIndex(int index)
    {
        //Debug.Log("selected index: " + index);
    }

    private void setColorOfFiled(TextMeshProUGUI entity, int value)
    {
        // if (value > 0) entity.color = new Color(0, 1, 0);
        // else if (value < 0) entity.color = new Color(1, 0, 0);
        // else entity.color = new Color(1, 1, 1);
    }

    private void setColorOfFiled01(TextMeshProUGUI entity)
    {
        int value = Convert.ToInt16(entity.text);
        if (value > 0) entity.color = new Color(0, 1, 0);
        else if (value < 0) entity.color = new Color(1, 0, 0);
        else entity.color = new Color(1, 1, 1);
    }

    private void setColorOfFiled02(TextMeshProUGUI entity, int value)
    {
        if (value > 0) entity.color = new Color(0, 1, 0);
        else if (value < 0) entity.color = new Color(1, 0, 0);
        else entity.color = new Color(1, 1, 1);
    }

    private void setSignOfFiled(TextMeshProUGUI entity, int value)
    {
        if (value > 0) entity.text = "+" + value;
        else if (value == 0) entity.text = "+" + value;
        else entity.text = "" + value;
    }

    private void updateOldField(TextMeshProUGUI entity, int value)
    {
        entity.text = (Convert.ToInt16(entity.text) + value).ToString();
    }

    public void upgradeNow()
    {
        OddsBurnHolder.SetActive(false);

        // Get Random Stats
        var randomUpgradeToken = UnityEngine.Random.Range(0, 3);
        Debug.Log("randomUpgradeToken: " + randomUpgradeToken);

<<<<<<< Updated upstream:Client/Assets/AaronUnity/AddCardToken.cs
    public void upgradeNow()
    {
        OddsBurnHolder.SetActive(false);

        imageHolder.SetActive(false);
        statsHolder.SetActive(true);
        btnCancel.SetActive(false);
        UpgradeHeroesHolder.SetActive(false);
        OptionButtonsHolder.SetActive(false);

        selectedBoth.Clear();
        selectionState = "selection_hero";
        TopTitle.text = "Choose Heroe";


        for (int i = 0; i < InstantiatedTokens.instantiatedObjects.Count; i++)
        {
            InstantiatedTokens.instantiatedObjects[i].gameObject.transform.GetChild(0).GetComponent<Button>().interactable = true;

            InstantiatedTokens.instantiatedObjects[i].gameObject.GetComponent<CardDisplay>().InactifCardBurn.SetActive(false);
        }

        randomUpgradeToken = UnityEngine.Random.Range(0, 3);

        Debug.Log(randomUpgradeToken);

        #region Get new Values

        if (randomUpgradeToken == 0)
        {
            InstantiatedTokens.cardWeapons = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(4).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardEngineering = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(5).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardBiotech = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(6).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardPsychics = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(7).GetComponent<TextMeshProUGUI>().text);
        }
        else if (randomUpgradeToken == 1)
        {
            InstantiatedTokens.cardWeapons = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(8).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardEngineering = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(9).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardBiotech = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(10).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardPsychics = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(11).GetComponent<TextMeshProUGUI>().text);
        }
        else if (randomUpgradeToken == 2)
        {
            InstantiatedTokens.cardWeapons = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(12).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardEngineering = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(13).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardBiotech = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(14).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardPsychics = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(15).GetComponent<TextMeshProUGUI>().text);
        }

        #endregion

        #region update selected card

        statsHolder.transform.GetChild(1).GetChild(0).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.cardWeapons.ToString();

        statsHolder.transform.GetChild(1).GetChild(1).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.cardEngineering.ToString();

        statsHolder.transform.GetChild(1).GetChild(2).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.cardBiotech.ToString();

        statsHolder.transform.GetChild(1).GetChild(3).GetComponentInChildren<TextMeshProUGUI>().text = InstantiatedTokens.cardPsychics.ToString();

        #endregion
    }

=======
        #region Get new Values
>>>>>>> Stashed changes:Client/Assets/AaronUnity/Scripts/AddCardToken.cs

        if (randomUpgradeToken == 0)
        {
            InstantiatedTokens.cardWeapons = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(4).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardEngineering = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(5).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardBiotech = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(6).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardPsychics = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(7).GetComponent<TextMeshProUGUI>().text);
        }
        else if (randomUpgradeToken == 1)
        {
            InstantiatedTokens.cardWeapons = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(8).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardEngineering = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(9).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardBiotech = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(10).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardPsychics = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(11).GetComponent<TextMeshProUGUI>().text);
        }
        else if (randomUpgradeToken == 2)
        {
            InstantiatedTokens.cardWeapons = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(12).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardEngineering = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(13).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardBiotech = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(14).GetComponent<TextMeshProUGUI>().text);
            InstantiatedTokens.cardPsychics = Convert.ToInt16(OddsBurnHolder_fieds.transform.GetChild(15).GetComponent<TextMeshProUGUI>().text);
        }

        #endregion

        // Burn & Remove them from List
        Debug.Log("selectedBoth: " + selectedBoth.Count);
        for (int i = 0; i < selectedBoth.Count; i++)
        {
            Token token1 = new Token()
            {
                id = selectedBoth[i]
            };

            Debug.Log("BurnedId: " + selectedBoth[i]);

            GameObject.FindObjectOfType<Rooster>().RemoveToken(token1);
            GameObject.FindObjectOfType<SelectionRooster>().Refresh();
        }

        selectedBoth.Clear();

        updateNewStats(InstantiatedTokens.cardWeapons,
                InstantiatedTokens.cardEngineering,
                InstantiatedTokens.cardBiotech,
                InstantiatedTokens.cardPsychics);

        Debug.Log("UpdatedCardBurnId: " + InstantiatedTokens.cardId);
        Debug.Log("cardWeapons: " + InstantiatedTokens.cardWeapons);
        Debug.Log("cardEngineering: " + InstantiatedTokens.cardEngineering);
        Debug.Log("cardBiotech: " + InstantiatedTokens.cardBiotech);
        Debug.Log("cardPsychics: " + InstantiatedTokens.cardPsychics);

        fx.Play();

        Token token = new Token()
        {
            id = tempGameObject.txtId.text,
            weapons = InstantiatedTokens.cardWeapons,
            engineering = InstantiatedTokens.cardEngineering,
            biotech = InstantiatedTokens.cardBiotech,
            psychics = InstantiatedTokens.cardPsychics

        };
        GameObject.FindObjectOfType<Rooster>().UpdateToken(token);
        GameObject.FindObjectOfType<SelectionRooster>().Refresh();

        // Initialize GridView  
        i = 0;
        selectionState = "selection_hero";
        selectedBoth = new List<string>();
        imageHolder.SetActive(false);
        statsHolder.SetActive(true);
        btnCancel.SetActive(false);
        UpgradeHeroesHolder.SetActive(false);
        OptionButtonsHolder.SetActive(false);
        TopTitle.text = "Choose Hero";
        for (int k = 0; k < CardsListUI.transform.childCount; k++)
            Destroy(CardsListUI.transform.GetChild(k).gameObject);

        thumbnailCreator.SetActive(true);
        int count = GameObject.FindObjectOfType<Rooster>().MyHeroes.Count;
        if (count > 0)
        {
            foreach (Token currentToken in GameObject.FindObjectOfType<Rooster>().MyHeroes)
            {
                StartCoroutine(readNewCardToken(currentToken));
            }
        }
        thumbnailCreator.SetActive(false);

    }

    public void sentToBattle()
    {
        GameObject.FindObjectOfType<BattleMaster>().testTwo();

        // GameObject.FindObjectOfType<WebGlBridge>().TriggerBattle();
        GameObject.FindObjectOfType<TransitionManager>().TransitionIntoArena();
        // GameObject.FindObjectOfType<BattleMaster>().OptimisticResponse(CardHolders[i].ToToken());
    }


}